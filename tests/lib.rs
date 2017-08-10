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
    dbc.connect("DoesntExist", "", "").unwrap();
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
    let dbc = dbc.connect("DoesntExist", "", "");
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
    let dbc = dbc.connect("PostgreSQL", "postgres", "");
    match dbc {
        Success(c) => assert_no_diagnostic(&c.disconnect()),
        Info(c) => assert_no_diagnostic(&c),
        Error(c) => assert_no_diagnostic(&c),
    };
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn query_result() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = Connection::with_parent(&env).unwrap();
    let dbc = dbc.connect("PostgreSQL", "postgres", "").unwrap();
    {
        let stmt = Statement::with_parent(&dbc).unwrap();
        let mut stmt = match stmt.exec_direct("SELECT * FROM information_schema.tables") {
            ReturnNoData::Success(s) | ReturnNoData::Info(s) => {
                assert_no_diagnostic(&s);
                s
            }
            ReturnNoData::NoData(_) => panic!("No Data"),
            ReturnNoData::Error(s) => panic!("{}", get_last_error(&s)),
        };
        assert_eq!(12, stmt.num_result_cols().unwrap());
        let stmt = loop {
            stmt = match stmt.fetch() {
                ReturnNoData::Success(s) => s,
                ReturnNoData::Info(s) => s,
                ReturnNoData::Error(s) => {
                    panic!("Error during fetching row: {}", get_last_error(&s))
                }
                ReturnNoData::NoData(s) => break s,
            };
        };
    }
    dbc.disconnect().unwrap();
}

/// Checks for a diagnstic record. Should one be present this function panics printing the contents
/// of said record.
fn assert_no_diagnostic(diag: &Diagnostics) {
    use std::str;
    let mut buffer = [0; 512];
    match diag.diagnostics(1, &mut buffer) {
        DiagReturn::Success(dr) | DiagReturn::Info(dr) => {
            panic!(
                "{}",
                str::from_utf8(&buffer[0..(dr.text_length as usize)]).unwrap()
            )
        }
        DiagReturn::Error => panic!("Error during fetching diagnostic record"),
        DiagReturn::NoData => (),
    }
}

fn get_last_error(diag: &Diagnostics) -> String {
    use std::str;
    let mut buffer = [0; 512];
    match diag.diagnostics(1, &mut buffer) {
        DiagReturn::Success(dr) | DiagReturn::Info(dr) => {
            str::from_utf8(&buffer[0..(dr.text_length as usize)])
                .unwrap()
                .to_owned()
        }
        DiagReturn::Error => panic!("Error during fetching diagnostic record"),
        DiagReturn::NoData => panic!("No diagnostic available"),
    }
}
