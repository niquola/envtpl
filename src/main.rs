// use std::env;
// use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::process;

fn main() {
    let stdin = io::stdin();

    let mut failed = false;
    let mut res = String::from("");

    for line in stdin.lock().lines() {
        let mut state = 0;
        let mut vr = String::from("");
        for c in line.unwrap().chars() {

            if state == 0 {
                if c == '{' {
                    state = 1;
                } else {
                    res.push(c);
                }
            } else if state == 1 {
                if c == '{' {
                    state = 2;
                } else {
                    state = 0;
                    res.push('{')
                }
            } else if state == 2 {
                if c == '{' {
                    println!("Invalid");
                } else if c == '}' {
                    state = 3;
                } else {
                    if c != ' ' {
                        vr.push(c);
                    }
                }
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
