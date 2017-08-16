//! Prints all datasources to the standard out.
extern crate odbc_safe;
extern crate odbc_sys;
use odbc_safe::*;
use odbc_sys::*;
use std::str::from_utf8;

fn main() {

    let env = Environment::new().unwrap();
    let mut env = env.declare_version_3().unwrap();

    let mut description = [0; 512];
    let mut attributes = [0; 512];

    println!("ODBC Drivers:");

    loop {
        let (description_length, attributes_length) =
            match env.drivers(SQL_FETCH_NEXT, &mut description, &mut attributes) {
                ReturnOption::Success(v) => v,
                ReturnOption::Info(_) => panic!("Buffers not large enough. Truncation occurred."),
                ReturnOption::NoData(()) => break,
                ReturnOption::Error(()) => {
                    panic!("Error occurred. Could use diagnostics to learn more")
                }
            };

        println!(
            "\tDescription: {}\n\tAttributes: {}\n",
            from_utf8(&description[..(description_length as usize)]).unwrap(),
            from_utf8(&attributes[..(attributes_length as usize)]).unwrap()
        );
    }
}
