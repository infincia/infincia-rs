#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(rustc_attrs)]
#![feature(custom_attribute)]
#![feature(custom_derive)]
#![feature(try_trait)]

/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

extern crate serde;

#[macro_use] extern crate serde_derive;

#[macro_use] extern crate serde_json;

extern crate walkdir;

extern crate pulldown_cmark;

extern crate number_prefix;

extern crate mime_sniffer;

extern crate crypto;

extern crate rand;

#[macro_use(log,debug,warn)]
extern crate log;

extern crate chrono;
extern crate chrono_humanize;

#[macro_use] extern crate lazy_static;

extern crate parking_lot;

#[macro_use] extern crate tera;

extern crate rocket;

extern crate rocket_contrib;

#[macro_use] extern crate diesel_codegen;

#[macro_use] extern crate diesel;

#[macro_use] extern crate error_chain;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate dotenv;

extern crate multipart;

extern crate image;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate url;

extern crate sys_info;

pub mod error;
pub mod schema;
mod public;
mod admin;
pub mod dbmodels;
pub mod apimodels;
mod database;
mod templates;
mod memcache;
mod util;
mod maintenance;

// public API
pub use ::rocket::config::Environment;

pub use dbmodels::*;

pub use maintenance::maintenance;

use ::database::DB_POOL;
use ::database::DB;

use ::rand::Rng;
use ::rand::thread_rng;


lazy_static! {
    pub static ref REGISTRATION_KEY: String = {
        let mut r = thread_rng();

        let registration_key: String = r.gen_ascii_chars().take(32).collect();

        registration_key
    };
}

lazy_static! {
    pub static ref WORKERS: ::parking_lot::RwLock<u16> = ::parking_lot::RwLock::new(0);
}

pub fn setup(workers: u16) {
    println!("setting up {} databbase connections", workers);
    {
        let mut dbpool = DB_POOL.write();
        *dbpool = Some(DB::create_db_pool(workers));
    }
    {
        let mut worker_v = WORKERS.write();
        *worker_v = workers;
    }
}

pub fn run(ip: &str, port: u16, workers: u16, environment: Environment, secret_key: &str) {
    println!("launching all with options: {}:{}", ip, port);

    let registration_key: &str = &REGISTRATION_KEY;

    println!("registration key: {}", registration_key);

    let limits = ::rocket::config::Limits::new()
         .limit("forms", 30 * 1024 * 1024)
         .limit("json", 3 * 1024 * 1024);

    // rocket config
    let config = ::rocket::config::Config::build(environment)
        .address(ip)
        .limits(limits)
        .port(port)
        .workers(workers)
        .secret_key(secret_key)
        .expect("configuration failed to build");

    println!("using config: {:?}", config);

    // start routes
    rocket::custom(config, false)
        .mount("/", routes![
    ::public::maintenance,
    ::public::static_route,
    ::public::home,
    ::public::about,
    ::public::contact,
    ::public::consulting,
    ::public::blog,
    ::public::blog_post,
    ::public::blog_with_tag,
    ::public::blog_feed,
    ::public::code,
    ::public::case_study_hypegram,
    ::public::case_study_slotsrace,
    ::public::case_study_codepoints,
    ::public::case_study_mifi_ios,
    ::public::case_study_mifi_osx,
    ::public::apps,
    ::public::apps_mifi_ios,
    ::public::apps_mifi_mac,
    ::public::apps_codepoints,
    ::public::file_info_route,
    ::public::file_download_route,
    ::public::file_preview_route])
        .mount("/admin", routes![
    ::admin::dashboard,
    ::admin::stats_route,
    ::admin::register,
    ::admin::post_register,
    ::admin::login,
    ::admin::logout,
    ::admin::users_list_route,
    ::admin::post_login,
    ::admin::post_list_route,
    ::admin::post_edit_route,
    ::admin::post_tags,
    ::admin::post_create_route,
    ::admin::post_update_route,
    ::admin::post_delete_route,
    ::admin::file_list_route,
    ::admin::file_upload_route,
    ::admin::file_refresh_route,
    ::admin::file_delete_route])
        .catch(errors![::public::not_found_route, ::public::internal_error_route, ::admin::authentication_required])
        .launch();
}

