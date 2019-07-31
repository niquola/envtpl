extern crate chrono;
extern crate base64;

use std::io;
use std::io::prelude::*;
use std::env;
use std::process;
use chrono::prelude::*;
use base64::{encode};


fn main() {
    let stdin = io::stdin();

    let mut failed = false;
    let mut res = String::from("");

    for line in stdin.lock().lines() {
        let mut state = 0;
        let mut vr = String::from("");
        let mut w = String::from("");
        for c in line.unwrap().chars() {
            // normal state
            if state == 0 {
                if c == '{' {
                    state = 1;
                } else {
                    res.push(c);
                }
            // second {
            } else if state == 1 {
                if c == '{' {
                    state = 2;
                } else {
                    state = 0;
                    res.push('{');
                    res.push(c);
                }
            // var name
            } else if state == 2 {
                if c == '{' {
                    println!("Invalid");
                } else if c == '}' {
                    state = 3;
                } else if c == '|' {
                    state = 4;
                } else if c == '#' {
                    state = 5;
                } else {
                    if c != ' ' {
                        vr.push(c);
                    }
                }
            //second }
            } else if state == 3 {
                if c == '}' {
                    match env::var(vr.clone()) {
                        Err(_e) => {
                            res.push_str("<<NOVAR:");
                            res.push_str(&vr);
                            res.push_str(">>");
                            eprintln!("No VAR: {}", vr);
                            failed = true;
                        },
                        Ok(value) => res.push_str(&value),
                    }
                    vr = String::from("");
                    state = 0;
                } else {
                    res.push('}');
                    state = 2;
                }
            //default
            } else if state == 4 {
                if c == '|' {
                    state = 41;
                } else {
                    w.push(c);
                    state = 6;
                }
            } else if state == 41 {
                if c == '}' {
                   state = 42;
                } else {
                    w.push(c);
                }
            } else if state == 42 {
                if c == '}' {
                    match env::var(vr.clone()) {
                        Err(_e) =>  res.push_str(w.trim()),
                        Ok(value) => res.push_str(&value),
                    }
                    vr = String::from("");
                    w = String::from("");
                    state = 0;
                } else {
                   w.push('}');
                   state = 41;
                }
            } else if state == 5 {
                if c == '}' {
                    state = 51;
                } else {
                    if c != ' ' {
                        w.push(c);
                    }
                }
            } else if state == 51 {
                if c == '}' {
                    if w == "inst" {
                        let now: DateTime<Utc> = Utc::now();
                        res.push_str(&now.to_rfc3339());
                    } else {
                        eprintln!("No fn: {}", w);
                        failed = true;
                    }
                    vr = String::from("");
                    w = String::from("");
                    state = 0;
                } else {
                    w.push('}');
                    state = 50;
                }
            } else if state == 6 {
                if c == '}' {
                    state = 61;
                } else {
                    if c != ' ' {
                        w.push(c);
                    }
                }
            } else if state == 61 {
                if c == '}' {
                    if w.trim() == "base64" {
                        match env::var(vr.clone()) {
                            Err(_e) => {
                                res.push_str("<<NOVAR:");
                                res.push_str(&vr);
                                res.push_str(">>");
                                eprintln!("No VAR: {}", vr);
                                failed = true;
                            },
                            Ok(value) => res.push_str(&encode(&value)),
                        }
                    } else {
                        eprintln!("No fn: {}", w);
                        failed = true;
                    }
                    vr = String::from("");
                    w = String::from("");
                    state = 0;
                } else {
                    w.push('}');
                    state = 6;
                }
            }
        }
        res.push('\n');
    }

    if failed {
        eprint!("{}", res);
        process::exit(1)
    } else {
        print!("{}", res);
        process::exit(0)
    }
}
