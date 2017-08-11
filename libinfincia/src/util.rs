/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use chrono::{Local, Utc, DateTime};
use chrono_humanize::HumanTime;
use ::number_prefix::{binary_prefix, Standalone, Prefixed};

use serde_json::to_value;

use rocket_contrib::{Value};
use std::collections::HashMap;

pub fn format_number(value: Value, _:HashMap<String, Value>) -> ::tera::Result<Value> {
    debug!("formatting number");
    let s = try_get_value!("format_number", "value", String, value);
    debug!("parsing {}", s);

    let input: f64 = s.parse().expect("expected a number");

    let english = match binary_prefix(input) {
        Standalone(bytes)   => format!("{} bytes", bytes),
        Prefixed(prefix, n) => format!("{:.2} {}B", n, prefix),
    };

    Ok(to_value(
        &english
    ).unwrap())

}

pub fn to_est(value: Value, _:HashMap<String, Value>) -> ::tera::Result<Value> {
    debug!("formatting EST date");

    let s = try_get_value!("to_est", "value", String, value);
    debug!("parsing {}", s);

    let utc = DateTime::parse_from_rfc2822(&s).expect("expected an RFC2822 date");
    debug!("utc {}", utc);


    let datetime = utc.with_timezone(&Local);
    debug!("datetime {}", datetime);

    let english = datetime.format("%b %e, %Y").to_string();
    debug!("english {}", english);

    Ok(to_value(
        &english
    ).unwrap())
}

pub fn to_est_full(value: Value, _:HashMap<String, Value>) -> ::tera::Result<Value> {
    debug!("formatting full EST date");

    let s = try_get_value!("to_est_full", "value", String, value);
    debug!("parsing {}", s);

    let utc = DateTime::parse_from_rfc2822(&s).expect("expected an RFC2822 date");
    debug!("utc {}", utc);

    let datetime = utc.with_timezone(&Local);
    debug!("datetime {}", datetime);

    let english = datetime.format("%b %e, %Y at %I:%M %p EST").to_string();
    debug!("english {}", english);

    Ok(to_value(
        &english
    ).unwrap())
}


pub fn to_relative(value: Value, _:HashMap<String, Value>) -> ::tera::Result<Value> {
    debug!("formatting relative date");

    let s = try_get_value!("to_relative", "value", String, value);
    debug!("parsing {}", s);

    let utc = DateTime::parse_from_rfc2822(&s).expect("expected an RFC2822 date");
    debug!("utc {}", utc);

    let datetime = utc.with_timezone(&Local);
    debug!("datetime {}", datetime);

    let now = Local::now();

    let duration = datetime.signed_duration_since(now);
    let ht = HumanTime::from(duration);
    let english = format!("{}", ht);
    debug!("english {}", english);

    Ok(to_value(
        &english
    ).unwrap())

}

pub fn current_time() -> String {
    let now = Utc::now();

    now.to_rfc2822()
}
