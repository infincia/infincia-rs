#![allow(unused_variables)]
#![allow(dead_code)]

/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use dbmodels::*;
use apimodels::{CurrentUser};

// admin

#[derive(Serialize)]
pub struct NotFoundTemplate<'a> {
    pub title: &'a str,
    pub error: Option<String>,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct InternalErrorTemplate<'a> {
    pub title: &'a str,
    pub error: Option<String>,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct RegisterTemplate<'a> {
    pub title: &'a str,
}

#[derive(Serialize)]
pub struct LoginTemplate<'a> {
    pub title: &'a str,
}

#[derive(Serialize)]
pub struct DashboardTemplate<'a> {
    pub title: &'a str,
    pub user: &'a CurrentUser,
    pub dashboard_selected: bool,
}

#[derive(Serialize)]
pub struct PostsTemplate<'a> {
    pub title: &'a str,
    pub user: &'a CurrentUser,
    pub posts: &'a [DBPost],
    pub postcount: usize,
    pub posts_selected: bool,
}

#[derive(Serialize)]
pub struct PostEditTemplate<'a> {
    pub title: &'a str,
    pub user: &'a CurrentUser,
    pub post: Option<DBPost>,
    pub posts_selected: bool,
    pub newpost: bool,
    pub editpost: bool,
}


#[derive(Serialize)]
pub struct FilesTemplate<'a> {
    pub title: &'a str,
    pub user: &'a CurrentUser,
    pub files: &'a [DBFile],
    pub filecount: usize,
    pub files_selected: bool,
}



#[derive(Serialize)]
pub struct UsersTemplate<'a> {
    pub title: &'a str,
    pub user: &'a CurrentUser,
    pub users: &'a [DBUser],
    pub userscount: usize,
    pub users_selected: bool,
}



// public

#[derive(Serialize)]
pub struct HomeTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct AboutTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub about_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct ContactTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub repositories: &'a [DBRepo],
    pub contact_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct BlogTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub postlist: &'a [DBPost],
    pub postcount: usize,
    pub repositories: &'a [DBRepo],

    pub blog_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct BlogPostTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub post: &'a DBPost,
    pub postcount: usize,
    pub repositories: &'a [DBRepo],
    pub blog_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct BlogFeedTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub description: &'a str,
    pub link: &'a str,
    pub lastbuilddate: &'a str,
}

#[derive(Serialize)]
pub struct CodeTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub repositories: &'a [DBRepo],
    pub code_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct FileInfoTemplate<'a> {
    pub title: &'a str,
    pub file: &'a DBFile,
    pub file_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct ConsultingTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub consulting_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct MaintenanceTemplate<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct CaseStudiesTemplate<'a> {
    pub title: &'a str,
    pub consulting_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct CaseStudyHypegramTemplate<'a> {
    pub title: &'a str,
    pub consulting_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct CaseStudySlotsraceTemplate<'a> {
    pub title: &'a str,
    pub consulting_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct CaseStudyCodepointsTemplate<'a> {
    pub title: &'a str,
    pub consulting_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct CaseStudyMiFiForiOSTemplate<'a> {
    pub title: &'a str,
    pub consulting_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct CaseStudyMiFiForOSXTemplate<'a> {
    pub title: &'a str,
    pub consulting_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct AppsTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub apps_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct AppsMiFIForiOSTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub apps_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct AppsMiFiForOSXTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub apps_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}

#[derive(Serialize)]
pub struct AppsCodepointsTemplate<'a> {
    pub title: &'a str,
    pub posts: &'a [DBPost],
    pub apps_selected: bool,
    pub description: &'a str,
    pub keywords: &'a str,
}