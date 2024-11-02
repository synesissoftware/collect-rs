// examples/text-file-char-freq.rs : example program that does a frequency analysis of characters in a (presumed to be) text file;

use collect_rs::containers::UnicodePointMap;

use std::{
    env as std_env,
    fs as std_fs,
    io as std_io,
    process as std_process,
    str as std_str,
};


fn main() -> Result<(), std_io::Error> {
    let mut process_path = None;
    let mut input_path = None;

    for arg in std_env::args() {
        if "--help" == arg {
            assert!(
                process_path.is_some(),
                "Rust runtime failed to provide program name as first element in `std::env::args`"
            );

            println!("USAGE: {} <input-path>", process_path.unwrap());

            return Ok(());
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
                },
            }
        }
    }

    process_file(&process_path.unwrap(), &input_path.unwrap())
}

fn process_file(
    process_path : &str,
    input_path : &str,
) -> Result<(), std_io::Error> {
    println!("processing '{input_path}'");

    let v = std_fs::read(input_path)?;

    let mut upm = UnicodePointMap::new('\u{10000}');

    match std_str::from_utf8(&v) {
        Ok(s) => {
            for c in s.chars() {
                upm.push(c);
            }
        },
        Err(e) => {
            return Err(std_io::ErrorKind::InvalidInput.into());
        },
    }

    println!("results ({}, {}):", upm.len(), upm.total());
    for (c, count) in upm.iter() {
        println!(
            "{} {} : {count}",
            c as u32,
            if c.is_control() {
                format!("{:#06x}", c as u32)
            } else {
                c.to_string()
            }
        );
    }

    Ok(())
}


// ///////////////////////////// end of file //////////////////////////// //
