//! Directly executes an SQL query and prints the result set to standard out
//!
//! This example also offers an idea, how to set up error handling for your ODBC Application.
extern crate odbc_safe;
extern crate odbc_sys;
use odbc_safe::*;
use odbc_sys::*;
use std::str::from_utf8;

// Actual application
fn main() {

    let env = Environment::new().unwrap();
    let mut env = env.declare_version_3().unwrap();

    let mut server_name = [0; 512];
    let mut description = [0; 512];

    println!("ODBC Data Sources:");

    loop {
        let (name_length, description_length) =
            match env.data_sources(SQL_FETCH_NEXT, &mut server_name, &mut description) {
                ReturnOption::Success(v) => v,
                ReturnOption::Info(_) => panic!("Buffers not large enough. Truncation occurred."),
                ReturnOption::NoData(()) => break,
                ReturnOption::Error(()) => {
                    panic!("Error occurred. Could use diagnostics to learn more")
                }
            };

        println!(
            "\tName: {}\n\tDescription: {}\n",
            from_utf8(&server_name[..(name_length as usize)]).unwrap(),
            from_utf8(&description[..(description_length as usize)]).unwrap()
        );

    }
}
