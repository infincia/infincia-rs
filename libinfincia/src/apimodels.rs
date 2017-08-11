/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use std::ops::Try;

use ::chrono::NaiveDateTime;
use ::chrono::Utc;

use rocket::http::Status;
use rocket::outcome::{IntoOutcome};
use rocket::{Request, Data};
use rocket::data::{self, FromData};
use rocket::request::{Outcome, FromRequest};

use rocket::Outcome::*;

use mime_sniffer::{MimeTypeSniffer};

use multipart::server::{Multipart};

use multipart::server::{MultipartData, ReadEntryResult};

use ::crypto::digest::Digest;
use ::crypto::sha2::Sha256;
use ::crypto::md5::Md5;

use ::database::DB;
use ::database::AdminDB;

#[derive(Debug, Serialize)]
pub struct CurrentUser {
    pub name: String,
    pub email: String,
    pub avatar: Option<Vec<u8>>,
}

impl From<::dbmodels::DBUser> for CurrentUser {
    fn from(user: ::dbmodels::DBUser) -> CurrentUser {
        CurrentUser {
            name: user.name,
            email: user.email,
            avatar: user.avatar,
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterCredentials {
    pub name: String,
    pub email: String,
    pub password: String,
    pub registration_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct APIPost {
    pub id: i32,
    pub url: String,
    pub tags: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct AllPosts {
    pub list: Vec<::dbmodels::DBPost>,
}

impl<'a, 'r> FromRequest<'a, 'r> for AllPosts {
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

        let list = match db.get_all_posts() {
            Ok(posts) => posts,
            Err(_) => {
                return None.into_outcome(
                    (Status::new(500, "database error"), ())
                );
            }
        };

        Ok(AllPosts {
            list: list,
        }).into_outcome(
            Status::new(200, "ok")
        )

    }
}

#[derive(Debug, Serialize)]
pub struct Repos {
    pub list: Vec<::dbmodels::DBRepo>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Repos {
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

        let list = match db.get_repos() {
            Ok(list) => list,
            Err(_) => {
                return None.into_outcome(
                    (Status::new(500, "database error"), ())
                );
            }
        };

        Ok(Repos {
            list: list,
        }).into_outcome(
            Status::new(200, "ok")
        )
    }
}

#[derive(Debug, Serialize)]
pub struct PublicPosts {
    pub list: Vec<::dbmodels::DBPost>,
}

impl<'a, 'r> FromRequest<'a, 'r> for PublicPosts {
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

        let list = match db.get_published_posts() {
            Ok(posts) => posts,
            Err(_) => {
                return None.into_outcome(
                    (Status::new(500, "database error"), ())
                );
            }
        };

        Ok(PublicPosts {
            list: list,
        }).into_outcome(
            Status::new(200, "ok")
        )

    }
}

#[derive(Debug, Deserialize)]
pub struct APIRepo {
    pub id: i32,
    pub html_url: String,
    pub name: String,
    pub created_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct APIFile {
    pub id: Option<i32>,
    pub name: String,
    pub mime_type: Option<String>,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub sha256: String,
    pub md5: String,
    pub data: Vec<u8>,
    pub preview: Option<Vec<u8>>,
    pub length: i64,
    pub has_preview: bool,
}

impl FromData for APIFile {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Ensure the content type is correct before opening the data.
        let content_type = match req.content_type() {
            Some(t) => t,
            None => return Failure((Status::BadRequest, "no content type".into()))
        };

        let boundary = match content_type.params().filter_map(|(key, value)| {
            match (key, value) {
                ("boundary", boundary) => Some(boundary),
                _ => None
            }
        }).last() {
            Some(boundary) => {
                boundary
            },
            None => {
                return ::rocket::outcome::Outcome::Forward(data);
            }
        };

        let mp = Multipart::with_body(data.open(), boundary);


        match mp.into_entry() {
            ReadEntryResult::Entry(entry) => {
                match entry.data {
                    MultipartData::Text(_) => {
                        println!("no file found");
                        return Failure((Status::BadRequest, "no file found".into()));
                    },
                    MultipartData::File(mut f) =>{
                        let filename = match f.filename {
                            Some(ref filename) => filename.clone(),
                            None => {
                                println!("no filename found");
                                return Failure((Status::BadRequest, "no filename found".into()))
                            },
                        };

                        let mut buf = Vec::new();
                        f.save().write_to(&mut buf);

                        let mime_type = {
                            match buf.as_slice().sniff_mime_type() {
                                Some(t) => {
                                    t.to_owned()
                                },
                                None => {
                                    "application/octet-stream".to_owned()
                                }
                            }
                        };

                        let sha256 = {
                            let mut hasher = Sha256::new();

                            hasher.input(buf.as_slice());

                            hasher.result_str()
                        };

                        let md5 = {
                            let mut hasher = Md5::new();

                            hasher.input(buf.as_slice());

                            hasher.result_str()
                        };

                        let length = {
                            buf.len()
                        };

                        let utc = Utc::now();
                        let now = utc.naive_utc();

                        let file = APIFile {
                            id: None,
                            name: filename.to_owned(),
                            mime_type: Some(mime_type.to_owned()),
                            date: now,
                            description: None,
                            sha256: sha256,
                            md5: md5,
                            data: buf,
                            preview: None,
                            length: length as i64,
                            has_preview: false,
                        };

                        Success(file)
                    },
                }
            },
            ReadEntryResult::End(_) => {
                println!("no file entries found in multipart data");
                Failure((Status::BadRequest, "no file entries found in multipart data".into()))
            },
            ReadEntryResult::Error(_, error) => {
                println!("error getting multipart entry: {}", error);
                Failure((Status::BadRequest, format!("error getting multipart entry: {}", error)))
            },
        }
    }
}