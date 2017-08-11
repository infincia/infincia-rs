/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use ::serde::{Serialize};
use ::serde::{Serializer};
use serde::ser::SerializeStruct;

use ::chrono::NaiveDateTime;
use ::chrono::Utc;
use ::chrono::DateTime;

use schema::*;

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize)]
#[table_name="users"]
pub struct DBUser {
    pub id: i32,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub name: String,
    pub avatar: Option<Vec<u8>>,
}


#[derive(Insertable, Debug)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub name: String,
    pub avatar: Option<Vec<u8>>,
}



#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[table_name="posts"]
pub struct DBPost {
    pub id: i32,
    pub url: String,
    pub tag_list: Vec<String>,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub published: bool,
    pub description: String,
    pub html: String,
}

impl Serialize for DBPost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut post = serializer.serialize_struct("DBPost", 10)?;
        post.serialize_field("id", &format!("{}", self.id))?;
        post.serialize_field("url", &self.url)?;

        post.serialize_field("tag_list", &self.tag_list)?;

        post.serialize_field("title", &self.title)?;

        post.serialize_field("content", &self.content)?;

        let utc_date: DateTime<Utc> = DateTime::from_utc(self.date, Utc);

        post.serialize_field("date", &utc_date.to_rfc2822())?;

        let utc_updated: DateTime<Utc> = DateTime::from_utc(self.updated, Utc);

        post.serialize_field("updated", &utc_updated.to_rfc2822())?;

        post.serialize_field("published", &format!("{}", self.published))?;

        post.serialize_field("description", &self.description)?;
        post.serialize_field("html", &self.html)?;

        post.end()
    }
}

#[derive(Insertable, Debug)]
#[table_name="posts"]
pub struct NewPost {
    pub url: String,
    pub tag_list: Vec<String>,
    pub title: String,
    pub content: String,
    pub date: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub published: bool,
    pub description: String,
    pub html: String,
}





#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[table_name="repos"]
pub struct DBRepo {
    pub id: i32,
    pub html_url: String,
    pub name: String,
    pub created_at: String,
    pub description: Option<String>,
}


impl Serialize for DBRepo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut struc = serializer.serialize_struct("DBRepo", 5)?;
        struc.serialize_field("id", &format!("{}", self.id))?;
        struc.serialize_field("html_url", &self.html_url)?;
        struc.serialize_field("name", &self.name)?;

        struc.serialize_field("created_at", &self.created_at)?;

        if let Some(ref description) = self.description {
            struc.serialize_field("description", description)?;
        } else {
            let o: Option<String> = None;
            struc.serialize_field("description", &format!("{:?}", o))?;
        }

        struc.end()
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name="repos"]
pub struct NewRepo {
    pub html_url: String,
    pub name: String,
    pub created_at: String,
    pub description: Option<String>,
}


#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[table_name="files"]
pub struct DBFile {
    pub id: i32,
    pub name: String,
    pub mime_type: String,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub sha256: String,
    pub md5: String,
    pub data: Vec<u8>,
    pub preview: Option<Vec<u8>>,
    pub length: i64,
    pub has_preview: bool,
}

impl Serialize for DBFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut struc = serializer.serialize_struct("DBFile", 9)?;
        struc.serialize_field("id", &format!("{}", self.id))?;
        struc.serialize_field("name", &self.name)?;
        struc.serialize_field("mime_type", &self.mime_type)?;
        let utc: DateTime<Utc> = DateTime::from_utc(self.date, Utc);

        struc.serialize_field("date", &utc.to_rfc2822())?;

        if let Some(ref description) = self.description {
            struc.serialize_field("description", description)?;
        } else {
            let o: Option<String> = None;
            struc.serialize_field("description", &format!("{:?}", o))?;
        }
        struc.serialize_field("sha256", &self.sha256)?;
        struc.serialize_field("md5", &self.md5)?;
        struc.serialize_field("has_preview", &self.has_preview)?;
        struc.serialize_field("length", &format!("{}", self.length))?;

        struc.end()
    }
}

#[derive(Insertable, Debug)]
#[table_name="files"]
pub struct NewFile {
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
