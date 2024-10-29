// examples/text-file-char-freq.rs : ;

use collect_rs::containers::UnicodePointMap;

use std::{
    env as std_env,
    fs as std_fs,
    process as std_process,
    str as std_str,
};


fn main() {

    let mut process_path = None;
    let mut input_path = None;

    for arg in std_env::args() {

        if "--help" == arg {
            assert!(process_path.is_some(), "Rust runtime failed to provide program name as first element in `std::env::args`");

            println!("USAGE: {} <input-path>", process_path.unwrap());

            return;
        } else {
            match process_path {
                None => {
                    process_path = Some(arg);
                },
                Some(_) => {
                    match input_path {
                        None => {
                            input_path = Some(arg);
                        },
                        Some(_) => {
                            eprintln!("{}: too many arguments; use --help for usage", process_path.unwrap());

                            std_process::exit(1);
                        },
                    }
                }
            }
        }
    }

    process_file(&process_path.unwrap(), &input_path.unwrap());
}

fn process_file(
    process_path : &str,
    input_path : &str,
) {
    println!("processing '{input_path}'");

    let v = std_fs::read(input_path).unwrap();

    let mut upm = UnicodePointMap::new('\u{10000}');

    for c in std_str::from_utf8(&v).unwrap().chars() {
        upm.push(c);
    }

    println!("results ({}, {}):", upm.len(), upm.total());
    for (c , count) in upm.iter() {
        println!("{} {} : {count}", c as u32, if c.is_control() { format!("{:#06x}", c as u32) } else { c.to_string() } );
    }
}


// ///////////////////////////// end of file //////////////////////////// //
