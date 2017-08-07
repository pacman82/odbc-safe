extern crate odbc_sys;
extern crate odbc_safe;

use odbc_safe::*;

#[test]
fn allocate_environment() {
    Environment::new().unwrap();
}

#[test]
fn allocate_connection() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    Connection::with_parent(&env).unwrap();
}

#[test]
#[should_panic]
fn wrong_datasource() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = Connection::with_parent(&env).unwrap();
    dbc.connect(b"DoesntExist" as &[u8], b"" as &[u8], b"" as &[u8])
        .unwrap();
}

#[test]
fn diagnostics() {

    let expected = if cfg!(target_os = "windows") {
        "[Microsoft][ODBC Driver Manager] Data source name not found and no default driver \
         specified"
    } else {
        "[unixODBC][Driver Manager]Data source name not found, and no default driver specified"
    };

    use std::str;

    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();

    let dbc = Connection::with_parent(&env).unwrap();
    let dbc = dbc.connect(b"DoesntExist" as &[u8], b"" as &[u8], b"" as &[u8]);
    if let Error(d) = dbc {
        let mut message = [0; 512];
        match d.diagnostics(1, &mut message) {
            DiagReturn::Success(rec) => {
                let message = str::from_utf8(&mut message[..(rec.text_length as usize)]).unwrap();
                assert_eq!(expected, message);
            }
            _ => panic!("Error retriving diagnostics"),
        }
    }
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn connect_to_postgres() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = Connection::with_parent(&env).unwrap();
    let dbc = dbc.connect(b"PostgreSQL" as &[u8], b"postgres" as &[u8], b"" as &[u8]);
    match dbc {
        Success(c) => panic_with_diagnostic(&c.disconnect()),
        Info(c) => panic_with_diagnostic(&c),
        Error(c) => panic_with_diagnostic(&c),
    };
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn query_result() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = Connection::with_parent(&env).unwrap();
    let dbc = dbc.connect(b"PostgreSQL" as &[u8], b"postgres" as &[u8], b"" as &[u8])
        .unwrap();
    {
        let stmt = Statement::with_parent(&dbc).unwrap();
        match stmt.exec_direct(b"SELECT * FROM information_schema.tables" as &[u8]) {
            ReturnNoData::Success(s) |
            ReturnNoData::Info(s) => {
                panic_with_diagnostic(&s);
                assert_eq!(12, s.num_result_cols().unwrap());
            }
            ReturnNoData::NoData(_) => panic!("No Data"),
            ReturnNoData::Error(s) => {
                panic_with_diagnostic(&s);
            }
        }
    }
    dbc.disconnect().unwrap();
}

/// Checks for a diagnstic record. Should one be present this function panics printing the contents
/// of said record.
fn panic_with_diagnostic(diag: &Diagnostics) {
    use std::str;
    let mut buffer = [0; 512];
    match diag.diagnostics(1, &mut buffer) {
        DiagReturn::Success(dr) |
        DiagReturn::Info(dr) => {
            panic!(
                "{}",
                str::from_utf8(&buffer[0..(dr.text_length as usize)]).unwrap()
            )
        }
        DiagReturn::Error => panic!("Error during fetching diagnostic record"),
        DiagReturn::NoData => (),
    }
}
