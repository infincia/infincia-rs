#![feature(use_extern_macros)]

/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

extern crate crossbeam;
use crossbeam::scope;

extern crate clap;
use clap::{Arg, App, SubCommand};

#[macro_use]
extern crate log;


extern crate simplelog;

use simplelog::{Config as LogConfig, TermLogger, SimpleLogger};

extern crate infincia;
use infincia::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const COPYRIGHT: &'static str = "(C) 2017 Infincia LLC";


fn main() {
    let app = App::new(NAME)
        .version(VERSION)
        .about(COPYRIGHT)
        .setting(::clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("serve")
            .about("serve website over http")
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("port to listen on")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("ip")
                .short("i")
                .long("ip")
                .value_name("IP")
                .help("IP to listen on")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("workers")
                .short("w")
                .long("workers")
                .value_name("WORKERS")
                .help("number of workers to start")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("key")
                .short("k")
                .long("key")
                .value_name("KEY")
                .help("secret key")
                .takes_value(true)
                .required(true)
            )
        )
        .arg(Arg::with_name("debug")
            .short("d")
            .help("debug logging, use twice for trace logging")
            .multiple(true)
        )
        .arg(Arg::with_name("production")
            .short("p")
            .conflicts_with("staging")
            .help("use the production environment")
        )
        .arg(Arg::with_name("staging")
            .short("s")
            .conflicts_with("production")
            .help("use the staging environment")
        );

    let matches = app.get_matches();

    let log_level = match matches.occurrences_of("debug") {
        0 => ::log::LogLevelFilter::Info,
        1 | _ => ::log::LogLevelFilter::Debug,
    };

    match TermLogger::init(log_level, LogConfig::default()) {
        Ok(_) => {

        },
        Err(err) => {
            println!("WARNING: failed to initialize term logger: {}", err);

            match SimpleLogger::init(log_level, LogConfig::default()) {
                Ok(_) => {

                },
                Err(err) => {
                    println!("WARNING: failed to initialize simple logger: {}", err);
                }
            }
        }
    }

    let mut environment: Environment = Environment::Staging;

    if matches.is_present("production") {
        info!("Environment: production");
        environment = Environment::Production;
    } else {
        info!("Environment: staging");
    }


    println!();

    if let Some(m) = matches.subcommand_matches("serve") {

        let ip = match m.value_of("ip") {
            Some(ip) => ip,
            None => {
                println!("failed to get ip");
                std::process::exit(1);
            }
        };

        let port: u16 = m.value_of("port").unwrap()
            .trim()
            .parse()
            .expect("Expected a number for port");

        let workers: u16 = m.value_of("workers").unwrap()
            .trim()
            .parse()
            .expect("Expected a number for workers");

        let secret_key = match m.value_of("key") {
            Some(key) => key,
            None => {
                println!("failed to get key");
                std::process::exit(1);
            }
        };

        setup(workers);

        scope(|scope| {
            scope.spawn(|| {
                println!("Running maintenance loop");
                match maintenance() {
                    Ok(()) => {

                    },
                    Err(err) => {
                        println!("WARNING: maintenance thread error: {}", err);
                    }
                }
            });
            scope.spawn(|| {
                println!("Running all services");
                run(&ip, port, workers, environment, secret_key);

            });
        });
    }
}