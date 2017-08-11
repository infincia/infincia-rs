/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use std::io::Cursor;
use std::path::PathBuf;
use std::str::FromStr;
use std::ffi::OsStr;

use rocket::response::Stream;
use rocket::response::Redirect;
use rocket::http::ContentType;
use rocket::Request;
use rocket::response::{self, Response, Responder};

use ::dbmodels::*;
use ::apimodels::{PublicPosts, Repos};

use ::templates::*;
use ::memcache::STATIC;
use ::util::*;
use ::database::DB;


use rocket::response::content;

use tera::Tera;

// errors
static FOUR_ZERO_FOUR_TEMPLATE: &'static str = include_str!("../../assets/templates/404.html");
static FIVE_HUNDRED_TEMPLATE: &'static str = include_str!("../../assets/templates/500.html");

// maintenance
static MAINTENANCE_TEMPLATE: &'static str = include_str!("../../assets/templates/maintenance.html");


// public
static ABOUT_TEMPLATE: &'static str = include_str!("../../assets/templates/about.html");
static HOME_TEMPLATE: &'static str = include_str!("../../assets/templates/home.html");

static KITTENS_TEMPLATE: &'static str = include_str!("../../assets/templates/kittens.html");

static SINGLE_POST_TEMPLATE: &'static str = include_str!("../../assets/templates/singlepost.html");
static BLOG_TEMPLATE: &'static str = include_str!("../../assets/templates/blog.html");
static FEED_TEMPLATE: &'static str = include_str!("../../assets/templates/feed.html");

static FILE_INFO_TEMPLATE: &'static str = include_str!("../../assets/templates/file.html");

static CODE_TEMPLATE: &'static str = include_str!("../../assets/templates/code.html");
static CONTACT_TEMPLATE: &'static str = include_str!("../../assets/templates/contact.html");
static CONSULTING_TEMPLATE: &'static str = include_str!("../../assets/templates/consulting.html");

static APPS_TEMPLATE: &'static str = include_str!("../../assets/templates/apps.html");
static MIFI_IOS_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/mifimonitor/ios.html");
static MIFI_MAC_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/mifimonitor/mac.html");
static CODEPOINTS_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/codepoints/mac.html");

// fragments
static HEADER_TEMPLATE: &'static str = include_str!("../../assets/templates/header.html");
static FOOTER_TEMPLATE: &'static str = include_str!("../../assets/templates/footer.html");
static SIDEBAR_TEMPLATE: &'static str = include_str!("../../assets/templates/sidebar.html");

static MIFI_DESCRIPTION_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/mifimonitor/description.html");
static MIFI_DEVICES_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/mifimonitor/devices.html");
static MIFI_FAQ_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/mifimonitor/faq.html");
static MIFI_FIRMWARE_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/mifimonitor/firmware.html");
static MIFI_RELEASENOTES_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/mifimonitor/releasenotes.html");

static CODEPOINTS_DESCRIPTION_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/codepoints/description.html");
static CODEPOINTS_FAQ_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/codepoints/faq.html");
static CODEPOINTS_RELEASENOTES_TEMPLATE: &'static str = include_str!("../../assets/templates/apps/codepoints/releasenotes.html");


// case studies

static CASE_STUDIES: &'static str = include_str!("../../assets/templates/case-studies.html");

static CASE_STUDY_HYPEGRAM_TEMPLATE: &'static str = include_str!("../../assets/templates/casestudies/hypegram.html");
static CASE_STUDY_SLOTSRACE_TEMPLATE: &'static str = include_str!("../../assets/templates/casestudies/slotsrace.html");
static CASE_STUDY_CODEPOINTS_TEMPLATE: &'static str = include_str!("../../assets/templates/casestudies/codepoints.html");
static CASE_STUDY_MIFI_MONITOR_IOS_TEMPLATE: &'static str = include_str!("../../assets/templates/casestudies/mi-fi-monitor-ios.html");
static CASE_STUDY_MIFI_MONITOR_OSX_TEMPLATE: &'static str = include_str!("../../assets/templates/casestudies/mi-fi-monitor-osx.html");

// Use globbing
lazy_static! {
    pub static ref TEMPLATES: Tera = {

        let mut tera = Tera::default();

        tera.add_raw_templates(vec![
            ("404.html", FOUR_ZERO_FOUR_TEMPLATE),
            ("500.html", FIVE_HUNDRED_TEMPLATE),
            ("header.html", HEADER_TEMPLATE),
            ("footer.html", FOOTER_TEMPLATE),
            ("sidebar.html", SIDEBAR_TEMPLATE),
            ("home.html", HOME_TEMPLATE),
            ("maintenance.html", MAINTENANCE_TEMPLATE),
            ("about.html", ABOUT_TEMPLATE),
            ("kittens.html", KITTENS_TEMPLATE),
            ("singlepost.html", SINGLE_POST_TEMPLATE),
            ("blog.html", BLOG_TEMPLATE),
            ("feed.html", FEED_TEMPLATE),
            ("file.html", FILE_INFO_TEMPLATE),
            ("code.html", CODE_TEMPLATE),
            ("contact.html", CONTACT_TEMPLATE),
            ("consulting.html", CONSULTING_TEMPLATE),
            ("apps.html", APPS_TEMPLATE),
            ("apps/mifimonitor/ios.html", MIFI_IOS_TEMPLATE),
            ("apps/mifimonitor/mac.html", MIFI_MAC_TEMPLATE),
            ("apps/codepoints/mac.html", CODEPOINTS_TEMPLATE),
            ("apps/mifimonitor/description.html", MIFI_DESCRIPTION_TEMPLATE),
            ("apps/mifimonitor/devices.html", MIFI_DEVICES_TEMPLATE),
            ("apps/mifimonitor/faq.html", MIFI_FAQ_TEMPLATE),
            ("apps/mifimonitor/firmware.html", MIFI_FIRMWARE_TEMPLATE),
            ("apps/mifimonitor/releasenotes.html", MIFI_RELEASENOTES_TEMPLATE),
            ("apps/codepoints/description.html", CODEPOINTS_DESCRIPTION_TEMPLATE),
            ("apps/codepoints/faq.html", CODEPOINTS_FAQ_TEMPLATE),
            ("apps/codepoints/releasenotes.html", CODEPOINTS_RELEASENOTES_TEMPLATE),
            ("case-studies.html", CASE_STUDIES),
            ("casestudies/hypegram.html", CASE_STUDY_HYPEGRAM_TEMPLATE),
            ("casestudies/slotsrace.html", CASE_STUDY_SLOTSRACE_TEMPLATE),
            ("casestudies/codepoints.html", CASE_STUDY_CODEPOINTS_TEMPLATE),
            ("casestudies/mi-fi-monitor-ios.html", CASE_STUDY_MIFI_MONITOR_IOS_TEMPLATE),
            ("casestudies/mi-fi-monitor-osx.html", CASE_STUDY_MIFI_MONITOR_OSX_TEMPLATE),
        ]).expect("Couldn't add templates");
        tera.autoescape_on(vec!["html"]);
        tera.register_filter("to_relative", to_relative);
        tera.register_filter("to_est", to_est);
        tera.register_filter("to_est_full", to_est_full);
        tera.register_filter("format_number", format_number);
        tera.autoescape_on(vec![]);

        tera
    };
}

pub struct StaticResource {
    pub content_type: ContentType,
    pub data: &'static [u8],
}

impl<'r> Responder<'r> for StaticResource {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .streamed_body(self.data)
            .header(self.content_type)
            .ok()
    }
}

#[error(404)]
pub fn not_found_route(_: &Request) -> content::Content<String> {
    let template = NotFoundTemplate {
        title: "Not found",
        error: None,
        description: "",
        keywords: "",
    };
    let template = TEMPLATES.render("404.html", &template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[error(500)]
pub fn internal_error_route(_: &Request) -> content::Content<String> {
    let template = InternalErrorTemplate {
        title: "Error",
        error: None,
        description: "",
        keywords: "",
    };
    let template = TEMPLATES.render("500.html", &template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/assets/<path..>")]
pub fn static_route(path: PathBuf) -> Option<StaticResource> {
    debug!("getting static file: {}", path.display());

    match STATIC.get(&path) {
        Some(f) => {
            let content_type = match path.extension() {
                Some(e) => {
                    let oext: &OsStr = e.as_ref();

                    let ext: Option<&str> = oext.to_str();

                    match ContentType::from_extension(ext.unwrap()) {
                        Some(c) => c,
                        None => {
                            ContentType::from_str("application/octet-stream").unwrap()
                        },
                    }
                },
                None => {
                    ContentType::from_str("application/octet-stream").unwrap()
                }
            };

            let resource = StaticResource {
                content_type: content_type,
                data: f,
            };

            Some(resource)
        },
        None => {
            None
        }
    }
}


#[get("/maintenance")]
pub fn maintenance() -> content::Content<String> {
    debug!("getting maintenance");

    let maintenance_template = MaintenanceTemplate {
        title: "Maintenance",
        description: "under maintenance",
        keywords: ""
    };
    let template = TEMPLATES.render("maintenance.html", &maintenance_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}


#[get("/")]
pub fn home() -> Redirect {

    Redirect::to("/apps")
}

#[get("/about")]
pub fn about(posts: PublicPosts) -> content::Content<String> {
    let about_template = AboutTemplate {
        about_selected: true,
        posts: posts.list.as_slice(),
        title: "About",
        description: "Infincia Software is a consulting firm located in central Ohio, specializing in iPhone, iPad, and Mac development.",
        keywords: "ios iphone ipad mac osx consulting freelancer developer",
    };
    let template = TEMPLATES.render("about.html", &about_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/contact")]
pub fn contact(posts: PublicPosts, repos: Repos) -> content::Content<String> {

    let contact_template = ContactTemplate {
        contact_selected: true,
        repositories: repos.list.as_slice(),
        posts: posts.list.as_slice(),
        title: "Contact",
        description: "Infincia Software Support and Contact information",
        keywords: "infincia software contact support",
    };

    let template = TEMPLATES.render("contact.html", &contact_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/consulting")]
pub fn consulting(posts: PublicPosts) -> content::Content<String> {

    let consulting_template = ConsultingTemplate {
        consulting_selected: true,
        posts: posts.list.as_slice(),
        title: "Consulting",
        description: "Infincia Software is a consulting firm located in central Ohio, specializing in iPhone, iPad, and Mac development.",
        keywords: "ios iphone ipad mac osx consulting freelancer developer",
    };

    let template = TEMPLATES.render("consulting.html", &consulting_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/blog")]
pub fn blog(posts: PublicPosts, repos: Repos) -> content::Content<String> {
    let blog_template = BlogTemplate {
        blog_selected: true,
        posts: posts.list.as_slice(),
        postlist: posts.list.as_slice(),
        postcount: posts.list.len(),
        repositories: repos.list.as_slice(),
        title: "Blog",
        description: "Infincia Software Blog",
        keywords: "infincia blog mac osx ios python objective-c developer app store",
    };

    let template = TEMPLATES.render("blog.html", &blog_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/blog/feed.rss")]
pub fn blog_feed(posts: PublicPosts) -> content::Content<String> {
    debug!("getting blog feed");

    let build_date = current_time();

    let feed_template = BlogFeedTemplate {
        title: "Infincia Software",
        posts: posts.list.as_slice(),
        description: "Infincia Software Blog",
        link: "https://infincia.com",
        lastbuilddate: &build_date,
    };

    let template = TEMPLATES.render("feed.html", &feed_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/blog/<url>")]
pub fn blog_post(db: DB, url: String, posts: PublicPosts, repos: Repos) -> Option<content::Content<String>> {
    debug!("getting blog posts");

    let post = match db.get_post_by_url(&url) {
        Ok(post) => {
            post
        },
        Err(e) => {
            warn!("failed to get post: {}", e);

            return None
        },
    };

    let blog_template = BlogPostTemplate {
        blog_selected: true,
        posts: posts.list.as_slice(),
        post: &post,
        postcount: posts.list.len(),
        repositories: repos.list.as_slice(),
        title: &url,
        description: "Infincia Software Blog",
        keywords: "infincia blog mac osx ios python objective-c developer app store",
    };

    let template = TEMPLATES.render("singlepost.html", &blog_template).unwrap();
    let html = ContentType::HTML;

    Some(content::Content(html, template))
}

#[get("/blog/tag/<tag>")]
pub fn blog_with_tag(db: DB, tag: String, posts: PublicPosts, repos: Repos) -> content::Content<String> {
    format!("Getting blog posts by tag {}", tag);

    let tagged_posts = match db.get_published_posts_for_tag(&tag) {
        Ok(posts) => {
            posts
        },
        Err(e) => {
            warn!("failed to get blog posts by tag: {}", e);

            Vec::new()
        },
    };

    let blog_template = BlogTemplate {
        blog_selected: true,
        posts: posts.list.as_slice(),
        postlist: tagged_posts.as_slice(),
        postcount: tagged_posts.len(),
        repositories: repos.list.as_slice(),
        title: "Blog",
        description: "Infincia Software Blog",
        keywords: "infincia blog mac osx ios python objective-c developer app store",
    };

    let template = TEMPLATES.render("blog.html", &blog_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/code")]
pub fn code(posts: PublicPosts, repos: Repos) -> content::Content<String> {
    let code_template = CodeTemplate {
        code_selected: true,
        posts: posts.list.as_slice(),
        repositories: repos.list.as_slice(),
        title: "Open Source",
        description: "Infincia Software Open Source Code",
        keywords: "infincia code open source mac osx ios python objective-c rust",
    };

    let template = TEMPLATES.render("code.html", &code_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

// case studies

#[get("/case-studies/hypegram")]
pub fn case_study_hypegram() -> content::Content<String> {
    let hypegram_template = CaseStudyHypegramTemplate {
        consulting_selected: true,
        title: "Case Study: Hypegram",
        description: "Case Studies: Hypegram for OS X",
        keywords: "mac iphone ipod ipad apps developer consulting objective-c appstore case study hypegram",
    };

    let template = TEMPLATES.render("casestudies/hypegram.html", &hypegram_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}


#[get("/case-studies/slotsrace")]
pub fn case_study_slotsrace() -> content::Content<String> {
    let slotsrace_template = CaseStudySlotsraceTemplate {
        consulting_selected: true,
        title: "Case Study: Slots Race",
        description: "Case Studies: Slots Race for iOS",
        keywords: "mac iphone ipod ipad apps developer consulting objective-c appstore case study slotsrace",
    };

    let template = TEMPLATES.render("casestudies/slotsrace.html", &slotsrace_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}


#[get("/case-studies/codepoints")]
pub fn case_study_codepoints() -> content::Content<String> {
    let codepoints_template = CaseStudyCodepointsTemplate {
        consulting_selected: true,
        title: "Case Study: Codepoints for OS X",
        description: "Case Studies: Codepoints for OS X",
        keywords: "mac iphone ipod ipad apps developer consulting objective-c appstore case study codepoints",
    };

    let template = TEMPLATES.render("casestudies/codepoints.html", &codepoints_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}


#[get("/case-studies/mi-fi-monitor-ios")]
pub fn case_study_mifi_ios() -> content::Content<String> {
    let mifi_ios_template = CaseStudyMiFiForiOSTemplate {
        consulting_selected: true,
        title: "Case Study: Mi-Fi Monitor for iOS",
        description: "Case Studies: Mi-Fi Monitor for iOS",
        keywords: "mac iphone ipod ipad apps developer consulting objective-c appstore case study mi-fi monitor",
    };

    let template = TEMPLATES.render("casestudies/mi-fi-monitor-ios.html", &mifi_ios_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/case-studies/mi-fi-monitor-osx")]
pub fn case_study_mifi_osx() -> content::Content<String> {
    let mifi_osx_template = CaseStudyMiFiForOSXTemplate {
        consulting_selected: true,
        title: "Case Study: Mi-Fi Monitor for OS X",
        description: "Case Studies: Mi-Fi Monitor for OS X",
        keywords: "mac iphone ipod ipad apps developer consulting objective-c appstore case study mi-fi monitor",
    };

    let template = TEMPLATES.render("casestudies/mi-fi-monitor-osx.html", &mifi_osx_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}


// apps

#[get("/apps")]
pub fn apps(posts: PublicPosts) -> content::Content<String> {

    let apps_template = AppsTemplate {
        apps_selected: true,
        posts: posts.list.as_slice(),
        title: "Apps",
        description: "Infincia Software Applications",
        keywords: "mac iphone ipod ipad apps developer objective-c appstore",
    };

    let template = TEMPLATES.render("apps.html", &apps_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/apps/mi-fi-monitor-ios")]
pub fn apps_mifi_ios(posts: PublicPosts) -> content::Content<String> {

    let app_template = AppsTemplate {
        apps_selected: true,
        posts: posts.list.as_slice(),
        title: "Mi-Fi Monitor for iOS",
        description: "Mi-Fi Monitor is an invaluable iPhone and iPad app to keep track of your Mi-Fi hotspot\'s signal and battery level",
        keywords: "mifi app iphone ipod ipad ios",
    };

    let template = TEMPLATES.render("apps/mifimonitor/ios.html", &app_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/apps/mi-fi-monitor-mac")]
pub fn apps_mifi_mac(posts: PublicPosts) -> content::Content<String> {

    let app_template = AppsTemplate {
        apps_selected: true,
        posts: posts.list.as_slice(),
        title: "Mi-Fi Monitor for OS X",
        description: "Mi-Fi Monitor is an invaluable Mac OS X app to keep track of your Mi-Fi hotspot\'s signal and battery level",
        keywords: "mifi app osx mac",
    };

    let template = TEMPLATES.render("apps/mifimonitor/mac.html", &app_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/apps/codepoints")]
pub fn apps_codepoints(posts: PublicPosts) -> content::Content<String> {

    let app_template = AppsTemplate {
        apps_selected: true,
        posts: posts.list.as_slice(),
        title: "Codepoints",
        description: "Codepoints is a Mac app that makes it easy to find and copy any Unicode character into the clipboard",
        keywords: "unicode app mac osx special characters codepoints emoji",
    };

    let template = TEMPLATES.render("apps/codepoints/mac.html", &app_template).unwrap();

    let html = ContentType::HTML;

    content::Content(html, template)
}

#[get("/files/info/<file_name>")]
pub fn file_info_route(db: DB, file_name: String) -> Option<content::Content<String>> {
    debug!("getting file info {}", file_name);

    let f: DBFile = match db.get_file(&file_name) {
        Ok(f) => {
            f
        },
        Err(_) => {
            return None
        },
    };

    let file_template = FileInfoTemplate {
        file_selected: true,
        file: &f,
        title: &(&f).name,
        description: "",
        keywords: "",
    };

    let template = TEMPLATES.render("file.html", &file_template).unwrap();

    let html = ContentType::HTML;

    Some(content::Content(html, template))
}

#[get("/files/download/<file_name>")]
pub fn file_download_route(db: DB, file_name: String) -> Option<Stream<Cursor<Vec<u8>>>> {
    debug!("getting file {}", file_name);

    let f: Vec<u8> = match db.get_file_data(&file_name) {
        Some(f) => {
            f
        },
        None => {
            return None
        },
    };

    let cursor = Cursor::new(f);

    Some(Stream::from(cursor))
}

#[get("/files/preview/<file_name>")]
pub fn file_preview_route(db: DB, file_name: String) -> Option<Stream<Cursor<Vec<u8>>>> {
    debug!("getting file preview {}", file_name);

    let f: Vec<u8> = match db.get_file_preview(&file_name) {
        Some(f) => {
            f
        },
        None => {
            return None
        },
    };

    let cursor = Cursor::new(f);

    Some(Stream::from(cursor))
}