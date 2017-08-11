/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use ::diesel::prelude::*;
use ::diesel::pg::PgConnection;
use ::diesel::ArrayExpressionMethods;

use ::dotenv::dotenv;
use ::r2d2_diesel::ConnectionManager;
use ::r2d2::{Pool, PooledConnection, GetTimeout};
use ::r2d2::config::Builder;
use rocket::request::{Request, Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;


use ::schema::*;
use ::error::{Error, ErrorKind};

use std::env;

use ::dbmodels::*;

lazy_static! {
    pub static ref DB_POOL: ::parking_lot::RwLock<Option<Pool<ConnectionManager<PgConnection>>>> = ::parking_lot::RwLock::new(None);
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    fn conn(&self) -> &PgConnection {
        &*self.0
    }

    pub fn get_connection() -> Result<Self, Error> {
        let pool = DB_POOL.read();
        match *pool {
            Some(ref pool) => {
                match pool.get() {
                    Ok(conn) => Ok(DB(conn)),
                    Err(_) => Err(ErrorKind::DatabasePoolError.into()),
                }
            },
            None => {
                panic!("attempt to use the db pool before setting");
            }
        }
    }

    pub fn create_db_pool(workers: u16) -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let builder = Builder::new();
        let config = builder.pool_size(workers as u32).build();

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::new(config, manager).expect("Failed to create pool.")
    }

    pub fn get_post_by_id(&self, post_id: i32) -> Result<DBPost, ::diesel::result::Error> {
        use ::schema::posts::dsl::*;

        let post = posts.filter(id.eq(post_id))
            .first(self.conn());

        post
    }

    pub fn get_post_by_url(&self, post_url: &str) -> Result<DBPost, ::diesel::result::Error> {
        use ::schema::posts::dsl::*;

        let post = posts.filter(url.eq(post_url))
            .first(self.conn());

        post
    }

    pub fn get_published_posts(&self) ->  Result<Vec<DBPost>, ::diesel::result::Error> {
        use ::schema::posts::dsl::*;

        let results = posts.filter(published.eq(true)).order(date.desc())
            .load::<DBPost>(self.conn());

        results
    }

    pub fn get_published_posts_for_tag(&self, tag: &str) ->  Result<Vec<DBPost>, ::diesel::result::Error> {
        use ::schema::posts::dsl::*;

        let results = posts.filter(published.eq(true)).order(date.desc()).filter(tag_list.contains(vec![tag]))
            .load::<DBPost>(self.conn());

        results
    }

    pub fn get_repos(&self) -> Result<Vec<DBRepo>, ::diesel::result::Error> {
        use ::schema::repos::dsl::*;

        let results = repos.order(created_at.desc()).load::<DBRepo>(self.conn());

        results
    }

    pub fn get_file(&self, file_name: &str) -> Result<DBFile, ::diesel::result::Error>  {
        use ::schema::files::dsl::*;

        let file = files.filter(name.eq(file_name))
            .first::<DBFile>(self.conn());

        file
    }

    pub fn get_file_data(&self, file_name: &str) -> Option<Vec<u8>> {
        use ::schema::files::dsl::*;

        let file = files.filter(name.eq(file_name))
            .first::<DBFile>(self.conn());

        match file {
            Ok(f) => {
                Some(f.data)
            },
            Err(_) => {
                None
            }
        }
    }

    pub fn get_file_preview(&self, file_name: &str) -> Option<Vec<u8>> {
        use ::schema::files::dsl::*;

        let file = files.filter(name.eq(file_name)).filter(has_preview.eq(true))
            .first::<DBFile>(self.conn());

        match file {
            Ok(f) => {
                if let Some(preview_data) = f.preview {
                    Some(preview_data)
                } else {
                    None
                }
            },
            Err(_) => {
                None
            }
        }
    }
}

pub trait AdminDB {
    fn create_user(&self, user: &NewUser) -> Result<Vec<DBUser>, ::diesel::result::Error>;
    fn get_user(&self, email: &str) -> Result<DBUser, ::diesel::result::Error>;
    fn get_users(&self) ->  Result<Vec<DBUser>, ::diesel::result::Error>;
    fn get_all_posts(&self) -> Result<Vec<DBPost>, ::diesel::result::Error>;
    fn update_post(&self, post: &DBPost) -> Result<(), ::diesel::result::Error>;
    fn create_post<'a>(&self, new_post: &NewPost) -> Result<Vec<DBPost>, ::diesel::result::Error>;
    fn delete_post(&self, post_id: i32) -> Result<usize, ::diesel::result::Error>;
    fn update_repos(&self, repos: Vec<NewRepo>) -> Result<(), ::diesel::result::Error>;
    fn get_files(&self) ->  Result<Vec<DBFile>, ::diesel::result::Error>;
    fn create_file<'a>(&self, new_file: &NewFile) -> Result<Vec<DBFile>, ::diesel::result::Error>;
    fn update_file(&self, file: &DBFile) -> Result<(), ::diesel::result::Error>;
    fn delete_file(&self, file_name: &str) -> Result<usize, ::diesel::result::Error>;
}

impl AdminDB for DB {

    fn get_user(&self, user_email: &str) -> Result<DBUser, ::diesel::result::Error>  {
        use ::schema::users::dsl::*;

        let user = users.filter(email.eq(user_email))
            .first::<DBUser>(self.conn());

        user
    }

    fn get_users(&self) ->  Result<Vec<DBUser>, ::diesel::result::Error> {
        use ::schema::users::dsl::*;

        let results = users.order(email.desc()).load::<DBUser>(self.conn());

        results
    }

    fn create_user(&self, user: &NewUser) -> Result<Vec<DBUser>, ::diesel::result::Error>  {
        use ::schema::users;

        let result = ::diesel::insert(user).into(users::table)
            .get_results::<DBUser>(self.conn());

        result
    }

    fn get_all_posts(&self) -> Result<Vec<DBPost>, ::diesel::result::Error> {
        use ::schema::posts::dsl::*;

        let results = posts.order(date.desc()).load::<DBPost>(self.conn());

        results
    }

    fn update_post(&self, post: &DBPost) -> Result<(), ::diesel::result::Error>  {
        use ::schema::posts::dsl::*;

        match ::diesel::update(posts.find(post.id())).set(post).execute(self.conn()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }

        //post.save_changes(self.conn())
    }

    fn create_post<'a>(&self, new_post: &NewPost) -> Result<Vec<DBPost>, ::diesel::result::Error> {
        use ::schema::posts::dsl::*;

        let result = ::diesel::insert(new_post)
            .into(posts)
            .get_results::<DBPost>(self.conn());

        result
    }

    fn delete_post(&self, post_id: i32) -> Result<usize, ::diesel::result::Error> {
        use ::schema::posts::dsl::*;

        let result = ::diesel::delete(posts.filter(id.eq(post_id)))
            .execute(self.conn());

        result
    }


    fn update_repos(&self, repo_list: Vec<NewRepo>) -> Result<(), ::diesel::result::Error> {
        use ::schema::repos::dsl::*;

        match ::diesel::delete(repos)
            .execute(self.conn()) {
            Ok(_) => {
                match ::diesel::insert(&repo_list)
                    .into(repos)
                    .execute(self.conn()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            },
            Err(err) => Err(err),
        }


    }

    fn get_files(&self) ->  Result<Vec<DBFile>, ::diesel::result::Error> {
        use ::schema::files::dsl::*;

        let results = files.order(date.desc()).load::<DBFile>(self.conn());

        results
    }

    fn create_file<'a>(&self, new_file: &NewFile) -> Result<Vec<DBFile>, ::diesel::result::Error> {
        let result = ::diesel::insert(new_file).into(files::table)
            .get_results::<DBFile>(self.conn());

        result
    }

    fn update_file(&self, file: &DBFile) -> Result<(), ::diesel::result::Error>  {
        use ::schema::files::dsl::*;

        match ::diesel::update(files.find(file.id())).set(file).execute(self.conn()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn delete_file(&self, file_name: &str) -> Result<usize, ::diesel::result::Error> {
        use ::schema::files::dsl::*;

        let result = ::diesel::delete(files.filter(name.eq(file_name)))
            .execute(self.conn());

        result
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool = DB_POOL.read();
        match *pool {
            Some(ref pool) => {
                match pool.get() {
                    Ok(conn) => Success(DB(conn)),
                    Err(e) => Failure((Status::InternalServerError, e)),
                }
            },
            None => {
                panic!("attempt to use the db pool before setting");
            }
        }
    }
}
