/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use std::collections::{HashMap, HashSet};
use std::ops::Try;

use rocket_contrib::{Json, Value};

use rocket::request::{Request, Outcome, FromRequest};
use rocket::response::{Redirect};
use rocket::http::Status;
use rocket::outcome::{IntoOutcome};

//use rocket::data::{self, FromData};
//use rocket::{Data};
use rocket::http::Cookies;
use rocket::http::Cookie;
use rocket::http::ContentType;

use ::crypto::bcrypt;
use ::crypto::util::fixed_time_eq;
use ::rand::Rng;
use ::rand::thread_rng;

use ::apimodels::{CurrentUser, LoginCredentials, RegisterCredentials, APIFile, APIPost, AllPosts};
use ::templates::*;
use ::util::*;
use ::error::{Error, ErrorKind};

use ::database::AdminDB;
use ::database::DB;

use ::REGISTRATION_KEY;
use ::WORKERS;

use rocket::response::content;

use tera::Tera;


use image::{GenericImage, ImageFormat};


use chrono::Local;
use ::pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};


// errors
static FOUR_ZERO_FOUR_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/404.html");
static FIVE_HUNDRED_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/500.html");

// fragments
static HEADER_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/header.html");
static FOOTER_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/footer.html");

// admin
static REGISTER_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/register.html");

static LOGIN_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/login.html");

static DASHBOARD_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/dashboard.html");

static POST_EDIT_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/edit.html");

static USERS_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/users.html");

static FILES_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/files.html");
static POSTS_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/postlist.html");

static TICKETS_TEMPLATE: &'static str = include_str!("../../assets/templates/admin/tickets.html");




// Use globbing
lazy_static! {
    pub static ref TEMPLATES: Tera = {

        let mut tera = Tera::default();

        tera.add_raw_templates(vec![
            ("admin/404.html", FOUR_ZERO_FOUR_TEMPLATE),
            ("admin/500.html", FIVE_HUNDRED_TEMPLATE),
            ("admin/header.html", HEADER_TEMPLATE),
            ("admin/footer.html", FOOTER_TEMPLATE),
            ("admin/register.html", REGISTER_TEMPLATE),
            ("admin/login.html", LOGIN_TEMPLATE),
            ("admin/dashboard.html", DASHBOARD_TEMPLATE),
            ("admin/edit.html", POST_EDIT_TEMPLATE),
            ("admin/users.html", USERS_TEMPLATE),
            ("admin/files.html", FILES_TEMPLATE),
            ("admin/postlist.html", POSTS_TEMPLATE),
            ("admin/tickets.html", TICKETS_TEMPLATE),
        ]).expect("Couldn't add templates");
        tera.autoescape_on(vec!["html"]);
        tera.register_filter("to_relative", to_relative);
        tera.register_filter("to_est", to_est);
        tera.register_filter("to_est_full", to_est_full);
        tera.register_filter("format_number", format_number);

        tera
    };
}

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let db = match DB::from_request(request).into_result() {
            Ok(db) => db,
            Err(_) => {
                return None.into_outcome(
                    (Status::new(500, "database error"), ())
                );
            }
        };

        request.cookies()
            .get_private("email")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|email: String| {
                db.get_user(&email).ok()
            })
            .map(|user| {
                CurrentUser::from(user)
            })
            .into_outcome(
                (Status::new(403, "login required"), ())
            )
    }
}

#[error(403)]
pub fn authentication_required<'r>(_: &'r Request) -> Redirect {
    Redirect::to("/admin/login")
}

#[get("/")]
pub fn dashboard(user: CurrentUser) -> content::Content<String> {
    let dashboard_template = DashboardTemplate {
        dashboard_selected: true,
        user: &user,
        title: "Dashboard",
    };

    let template = TEMPLATES.render("admin/dashboard.html", &dashboard_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/register")]
pub fn register() -> content::Content<String> {
    println!("getting register");

    let register_template = RegisterTemplate {
        title: "Register",
    };

    let template = TEMPLATES.render("admin/register.html", &register_template).unwrap();


    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/login")]
pub fn login() -> content::Content<String> {
    println!("getting login");

    let login_template = LoginTemplate {
        title: "Login",
    };

    let template = TEMPLATES.render("admin/login.html", &login_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    println!("getting logout");

    cookies.remove_private(Cookie::named("email"));


    Redirect::to("/admin/login")
}

#[get("/users")]
pub fn users_list_route(db: DB, user: CurrentUser) -> content::Content<String> {
    println!("getting file list");

    let users = match db.get_users() {
        Ok(users) => {
            users
        }
        Err(_) => {
            Vec::new()
        }
    };

    let users_template = UsersTemplate {
        users_selected: true,
        users: users.as_slice(),
        userscount: users.len(),
        user: &user,
        title: "Users",
    };

    let template = TEMPLATES.render("admin/users.html", &users_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}


#[get("/posts")]
pub fn post_list_route(user: CurrentUser, posts: AllPosts) -> content::Content<String> {
    println!("getting blog posts");

    let posts_template = PostsTemplate {
        posts_selected: true,
        posts: posts.list.as_slice(),
        postcount: posts.list.len(),
        user: &user,
        title: "Blog posts",
    };

    let template = TEMPLATES.render("admin/postlist.html", &posts_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[derive(FromForm)]
pub struct SearchQuery {
    search: String,
}

#[get("/posts/tags?<query>")]
pub fn post_tags(user: CurrentUser, query: SearchQuery, posts: AllPosts) -> Json<Value> {
    let search: &str = query.search.as_ref();
    let _ = user;

    let tag_list: HashSet<String> = posts.list.into_iter()
        .flat_map( |post| post.tag_list.into_iter())
        .collect();


    let tags: Vec<HashMap<String, String>> = tag_list.into_iter()
        .filter(|tag| {
            tag.contains(search)
        })
        .map(|tag| {
            let mut map: HashMap<String, String> = HashMap::new();
            map.insert("value".to_string(), tag.clone());
            map.insert("text".to_string(), tag.clone());

            map
        })
        .collect();

    return Json(
        json!(tags)
    );
}

#[get("/posts/<post_id>")]
pub fn post_edit_route(db: DB, user: CurrentUser, post_id: i32) -> content::Content<String> {

    let edit_template = match db.get_post_by_id(post_id) {
        Ok(post) => {
            println!("editing blog post");

            PostEditTemplate {
                posts_selected: true,
                post: Some(post),
                title: "Edit post",
                user: &user,
                newpost: false,
                editpost: true,
            }
        }
        Err(_) => {
            println!("creating new blog post");

            PostEditTemplate {
                posts_selected: true,
                post: None,
                title: "New post",
                user: &user,
                newpost: true,
                editpost: false,
            }
        }
    };


    let template = TEMPLATES.render("admin/edit.html", &edit_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}


#[get("/files")]
pub fn file_list_route(db: DB, user: CurrentUser) -> content::Content<String> {
    println!("getting file list");

    let files = match db.get_files() {
        Ok(files) => {
            files
        }
        Err(_) => {
            Vec::new()
        }
    };

    let files_template = FilesTemplate {
        files_selected: true,
        files: files.as_slice(),
        filecount: files.len(),
        user: &user,
        title: "Files",
    };


    let template = TEMPLATES.render("admin/files.html", &files_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}


// API Requests

#[get("/stats")]
pub fn stats_route(user: CurrentUser) -> Json<Value> {
    let _ = user;

    println!("getting stats");

    let mut map = HashMap::<&str, Value>::new();

    if let Ok(cpu) = ::sys_info::cpu_speed() {
        map.insert("cpu_freq", Value::from(cpu));
    }

    if let Ok(mem) = ::sys_info::mem_info() {
        let total = mem.total;
        let free = mem.free;
        let used = total - free;
        let avail = mem.avail;
        let buffers = mem.buffers;
        let cached = mem.cached;
        let swap_total = mem.swap_total;
        let swap_free = mem.swap_free;

        map.insert("mem_total", Value::from(total));
        map.insert("mem_free", Value::from(free));
        map.insert("mem_used", Value::from(used));

        map.insert("swap_total", Value::from(swap_total));
        map.insert("swap_free", Value::from(swap_free));

        map.insert("mem_avail", Value::from(avail));

        map.insert("mem_buffers", Value::from(buffers));

        map.insert("mem_cached", Value::from(cached));


    }


    if let Ok(disk) = ::sys_info::disk_info() {
        let total = disk.total;
        let free = disk.free;
        let used = total - free;

        map.insert("disk_total", Value::from(total));
        map.insert("disk_free", Value::from(free));
        map.insert("disk_used", Value::from(used));

    }

    if let Ok(hostname) = ::sys_info::hostname() {
        map.insert("hostname", Value::from(hostname));
    }

    if let Ok(load) = ::sys_info::loadavg() {
        let one = load.one;
        let five = load.five;
        let fifteen = load.fifteen;

        map.insert("load_one", Value::from(one));
        map.insert("load_five", Value::from(five));
        map.insert("load_fifteen", Value::from(fifteen));
    }

    let app_version: &'static str = env!("CARGO_PKG_VERSION");

    map.insert("app_version", Value::from(app_version));

    if let Ok(os_version) = ::sys_info::os_release() {
        map.insert("os_version", Value::from(os_version));
    }

    let worker_v = WORKERS.read();

    map.insert("app_workers", Value::from(*worker_v));

    Json(json!(map))
}

#[post("/register", format = "application/json", data = "<credentials>")]
pub fn post_register(db: DB, credentials: Json<RegisterCredentials>) -> Result<(), Error> {
    println!("register");

    let cred: RegisterCredentials = credentials.into_inner();

    let registration_key: &str = &REGISTRATION_KEY;

    if !fixed_time_eq(&registration_key.as_bytes(), cred.registration_key.as_bytes()) {
        return Err(ErrorKind::InvalidRegistrationKey.into())
    }

    let mut stored_hash_bytes: [u8; 24] = [0; 24];

    let mut r = thread_rng();

    let salt: Vec<u8> = r.gen_iter::<u8>().take(16).collect();

    bcrypt::bcrypt(12, salt.as_slice(), cred.password.as_bytes(), &mut stored_hash_bytes);

    let user = ::dbmodels::NewUser {
        name: cred.name,
        email: cred.email,
        password_hash: stored_hash_bytes.to_vec(),
        password_salt: salt,
        avatar: None,
    };

    match db.create_user(&user) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("register error: {}", e);

            Err(ErrorKind::RegistrationFailed.into())
        }
    }
}

#[post("/login", format = "application/json", data = "<credentials>")]
pub fn post_login(db: DB, credentials: Json<LoginCredentials>, mut cookies: Cookies) -> Result<(), Error> {
    println!("login");

    let cred: LoginCredentials = credentials.into_inner();

    match db.get_user(&cred.email) {
        Ok(user) => {
            let mut check_hash_bytes: [u8; 24] = [0; 24];

            bcrypt::bcrypt(12, &user.password_salt, cred.password.as_bytes(), &mut check_hash_bytes);

            let stored_hash_bytes = user.password_hash.as_slice();

            if fixed_time_eq(&check_hash_bytes, stored_hash_bytes) {
                cookies.add_private(Cookie::new("email", user.email));

                Ok(())
            } else {
                return Err(ErrorKind::LoginFailed.into())
            }
        }
        Err(e) => {
            println!("login error: {}", e);

            return Err(ErrorKind::LoginFailed.into())
        }
    }
}


#[allow(unused_variables)]
#[post("/posts", format = "application/json", data = "<post>")]
pub fn post_create_route(db: DB, post: Json<APIPost>, user: CurrentUser) -> Result<(), Error> {
    let api_post: APIPost = post.into_inner();

    println!("creating blog post");

    let local = Local::now();
    let now = local.naive_local();

    let tag_list = api_post.tags.split(",").map(|t: &str| {
        t.replace(" ", "-")
    }).collect::<Vec<String>>();

    let mut html_rendered = String::new();

    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    opts.insert(OPTION_ENABLE_FOOTNOTES);

    let p = Parser::new_ext(&api_post.content, opts);
    html::push_html(&mut html_rendered, p);


    let post = ::dbmodels::NewPost {
        url: api_post.url,
        tag_list: tag_list,
        title: api_post.title,
        content: api_post.content.clone(),
        date: now,
        updated: now,
        published: api_post.published,
        description: api_post.description,
        html: html_rendered,
    };


    match db.create_post(&post) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("create post error: {}", e);

            return Err(ErrorKind::SaveFailed.into())
        }
    }
}

#[allow(unused_variables)]
#[put("/posts", format = "application/json", data = "<post>")]
pub fn post_update_route(db: DB, post: Json<APIPost>, user: CurrentUser) -> Result<(), Error> {
    let updated_post: APIPost = post.into_inner();

    let id = updated_post.id;

    println!("updating blog post: {}", id);

    let mut post = match db.get_post_by_id(id) {
        Ok(post) => post,
        Err(e) => {
            return Err(ErrorKind::SaveFailed.into())
        }
    };

    let local = Local::now();
    let now = local.naive_local();

    let mut html_rendered = String::new();

    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    opts.insert(OPTION_ENABLE_FOOTNOTES);

    let p = Parser::new_ext(&updated_post.content, opts);
    html::push_html(&mut html_rendered, p);

    let tag_list = updated_post.tags.split(",").map(|t: &str| {
        t.replace(" ", "-")
    }).collect::<Vec<String>>();


    //post.url = updated_post.url;
    post.tag_list = tag_list;
    post.title = updated_post.title;
    post.content = updated_post.content.clone();
    post.updated = now;
    post.published = updated_post.published;
    post.description = updated_post.description;
    post.html = html_rendered;

    match db.update_post(&post) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("update post error: {}", e);

            return Err(ErrorKind::SaveFailed.into())
        }
    }
}

#[allow(unused_variables)]
#[delete("/posts/<post_id>")]
pub fn post_delete_route(db: DB, post_id: i32, user: CurrentUser) -> Result<(), Error> {
    println!("deleting post {}", post_id);

    match db.delete_post(post_id) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("delete post: {}", e);

            return Err(ErrorKind::DeleteFailed.into())
        }
    }
}

#[allow(unused_variables)]
#[post("/files", data = "<file>")]
pub fn file_upload_route(db: DB, file: APIFile, user: CurrentUser) -> Result<(), Error> {
    println!("uploaded file");

    let new_file = ::dbmodels::NewFile {
        name: file.name,
        mime_type: file.mime_type,
        date: file.date,
        description: file.description,
        sha256: file.sha256,
        md5: file.md5,
        data: file.data,
        preview: file.preview,
        length: file.length,
        has_preview: file.has_preview,
    };

    match db.create_file(&new_file) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("upload error: {}", e);

            return Err(ErrorKind::UploadFailed.into())
        }
    }
}

#[allow(unused_variables)]
#[post("/files/refresh/<file_name>")]
pub fn file_refresh_route(db: DB, file_name: String, user: CurrentUser) -> Result<(), Error> {
    println!("refresh file");

    let mut file: ::dbmodels::DBFile = match db.get_file(&file_name) {
        Ok(file) => {
            file
        },
        Err(err) => {
            return Err(ErrorKind::FileNotFound.into())
        },
    };

    let size = file.data.len();

    let image = match ::image::load_from_memory(file.data.as_ref()) {
        Ok(image) => {
            image
        },
        Err(err) => {
            return Err(ErrorKind::FileNotFound.into())
        }
    };

    let width = image.width();
    let height = image.height();

    let preview = image.resize((width/2), (height/2), ::image::FilterType::Lanczos3);

    let mut preview_data: Vec<u8> = Vec::new();

    match preview.save( &mut preview_data, ImageFormat::JPEG) {
        Ok(_) => {

        },
        Err(err) => {
            return Err(ErrorKind::SaveFailed.into())
        }
    }

    file.preview = Some(preview_data);
    file.length = size as i64;
    file.has_preview = true;

    match db.update_file(&file) {
        Ok(()) => {
            Ok(())
        }
        Err(e) => {
            println!("resize error: {}", e);

            return Err(ErrorKind::SaveFailed.into())
        }
    }
}

#[allow(unused_variables)]
#[delete("/files/<file_name>")]
pub fn file_delete_route(db: DB, file_name: String, user: CurrentUser) -> Result<(), Error> {
    println!("deleting file {}", file_name);

    match db.delete_file(&file_name) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("delete file error: {}", e);

            return Err(ErrorKind::DeleteFailed.into())
        }
    }
}