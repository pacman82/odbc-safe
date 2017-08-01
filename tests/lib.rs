extern crate odbc_sys;
extern crate odbc_safe;

use odbc_safe::*;

#[test]
fn query_result() {
    let env = Environment::allocate().warning_as_error().unwrap();
    let env: Environment<Odbc3m8> = env.declare_version().warning_as_error().unwrap();
    Connection::with_parent(&env).warning_as_error().unwrap();
}

#[test]
fn wrong_datasource() {
    let env = Environment::allocate().warning_as_error().unwrap();
    let env: Environment<Odbc3m8> = env.declare_version().warning_as_error().unwrap();
    let dbc = Connection::with_parent(&env).warning_as_error().unwrap();
    let dbc = dbc.connect(b"DoesntExist".as_ref(), b"".as_ref(), b"".as_ref())
        .map_error(|_| ())
        .warning_as_error()
        .unwrap_err();
}

// #[test]
// fn diagnostics() {
//     use std::str;

//     let env = Environment::allocate().warning_as_error().unwrap();
//     let env: Environment<Odbc3m8> = env.declare_version().warning_as_error().unwrap();
//     let dbc = Connection::with_parent(&env).warning_as_error().unwrap();
//     if let Error(dbc) = dbc.connect(b"DoesntExist".as_ref(), b"".as_ref(), b"".as_ref()) {
//         let mut message = [0; 512];
//         match dbc.diagnostics(1, &mut message) {
//             DiagReturn::Success(_) => println!("{}", str::from_utf8(&mut message).unwrap()),
//             _ => panic!("No Diagnostics"),
//         }
//     }
// }
