extern crate odbc_safe;
extern crate odbc_sys;

use std::cell::RefCell;

use odbc_safe::*;

#[test]
fn allocate_environment() {
    Environment::new().unwrap();
}

#[test]
fn allocate_connection() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    DataSource::with_parent(&env).unwrap();
}

#[test]
#[should_panic]
fn wrong_datasource() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = DataSource::with_parent(&env).unwrap();
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

    let dbc = DataSource::with_parent(&env).unwrap();
    let dbc = dbc.connect("DoesntExist", "", "");
    if let Error(d) = dbc {
        let mut message = [0; 512];
        match d.diagnostics(1, &mut message) {
            ReturnOption::Success(rec) => {
                let message = str::from_utf8(&mut message[..(rec.text_length as usize)]).unwrap();
                assert_eq!(expected, message);
            }
            _ => panic!("Error retriving diagnostics"),
        }
    }
}

#[test]
fn drivers_with_empty_buffer() {
    use odbc_sys::SQL_FETCH_NEXT;
    let env = Environment::new().unwrap();
    let mut env: Environment<Odbc3> = env.declare_version().unwrap();
    let mut description = [0; 0];
    let mut attributes = [0; 0];
    match env.drivers(SQL_FETCH_NEXT, &mut description, &mut attributes) {
        ReturnOption::Error(()) => panic!("SQLDrivers call returned error"),
        _ => (),
    }
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn connect_to_postgres() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = DataSource::with_parent(&env).unwrap();
    let dbc = dbc.connect("PostgreSQL", "postgres", "");
    match dbc {
        Success(c) => assert_no_diagnostic(&c.disconnect()),
        Info(c) => assert_no_diagnostic(&c),
        Error(c) => assert_no_diagnostic(&c),
    };
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn connect_to_postgres_with_connection_string() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = DataSource::with_parent(&env).unwrap();
    let dbc = dbc.connect_with_connection_string("DSN=PostgreSQL;UID=postgres");
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
    let dbc = DataSource::with_parent(&env).unwrap();
    let dbc = dbc.connect("PostgreSQL", "postgres", "").unwrap();
    {
        let stmt = Statement::with_parent(&dbc).unwrap();
        let stmt = match stmt.exec_direct("SELECT title FROM Movies WHERE year=1968;") {
            ReturnOption::Success(s) |
            ReturnOption::Info(s) => {
                assert_no_diagnostic(&s);
                s
            }
            ReturnOption::NoData(_) => panic!("No Data"),
            ReturnOption::Error(s) => panic!("{}", get_last_error(&s)),
        };
        assert_eq!(1, stmt.num_result_cols().unwrap());
        let mut stmt = match stmt.fetch() {
            ReturnOption::Success(s) => s,
            ReturnOption::Info(s) => s,
            ReturnOption::Error(s) => panic!("Error during fetching row: {}", get_last_error(&s)),
            ReturnOption::NoData(_) => panic!("Empty result set returned from SELECT"),
        };
        let mut buffer = [0u8; 256];
        if let ReturnOption::Success(Indicator::Length(i)) =
            stmt.get_data(1, &mut buffer as &mut [u8])
        {
            assert_eq!("2001: A Space Odyssey".as_bytes(), &buffer[..(i as usize)]);
        } else {
            panic!("No field found!");
        }
    }
    dbc.disconnect().unwrap();
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn describe_result() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = DataSource::with_parent(&env).unwrap();
    let dbc = dbc.connect("PostgreSQL", "postgres", "").unwrap();
    {
        let stmt = Statement::with_parent(&dbc).unwrap();
        let mut stmt = match stmt.exec_direct("SELECT title, year FROM Movies") {
            ReturnOption::Success(s) |
            ReturnOption::Info(s) => s,
            _ => panic!("Did not return Result Set"),
        };
        let mut buffer = [0u8; 6];
        let mut indicator = 0;
        let mut nullable = odbc_sys::SQL_NO_NULLS;
        let data_type = stmt.describe_col(1, &mut buffer[..], &mut indicator, &mut nullable)
            .unwrap();
        println!("DataType {:?}", data_type);
        assert_eq!(data_type, Some(DataType::Varchar(255)));
        assert_eq!(&buffer, b"title\0");
        assert_eq!(indicator, 5);
        assert_eq!(nullable, odbc_sys::SQL_NULLABLE);
    }
    dbc.disconnect().unwrap();
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn auto_disconnect() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = DataSource::with_parent(&env).unwrap();
    dbc.connect("PostgreSQL", "postgres", "").unwrap();
    // No panic on Drop, because of automatic disconnect
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn not_read_only() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = DataSource::with_parent(&env).unwrap();
    let mut dbc = dbc.connect("PostgreSQL", "postgres", "").unwrap();
    assert!(!dbc.is_read_only().unwrap());
}

#[cfg_attr(not(feature = "travis"), ignore)]
#[test]
fn reuse_param_col_bindings() {
    let env = Environment::new().unwrap();
    let env: Environment<Odbc3> = env.declare_version().unwrap();
    let dbc = DataSource::with_parent(&env).unwrap();
    let dbc = dbc.connect("PostgreSQL", "postgres", "").unwrap();
    {
        let stmt = Statement::with_parent(&dbc).unwrap();
        match stmt.exec_direct( "CREATE TEMPORARY TABLE tbl (x INT NOT NULL, y INT NOT NULL, z INT NOT NULL);" ) {
            ReturnOption::Success(s) |
            ReturnOption::Info(s) => {
                assert_no_diagnostic(&s);
            }
            ReturnOption::NoData(s) => {
                assert_no_diagnostic(&s);
            }
            ReturnOption::Error(s) => panic!("{}", get_last_error(&s)),
        };
    }
    {
        let x: RefCell<i32> = RefCell::new(0);
        let y: RefCell<i32> = RefCell::new(0);
        let z: RefCell<i32> = RefCell::new(0);

        let stmt = Statement::with_parent(&dbc).unwrap();
        let stmt = stmt.prepare("INSERT INTO tbl (x,y,z) VALUES (?,?,?);").unwrap();
        let stmt = stmt.bind_input_parameter(1, DataType::Integer, &x, None).unwrap();
        let stmt = stmt.bind_input_parameter(2, DataType::Integer, &y, None).unwrap();
        let stmt = stmt.bind_input_parameter(3, DataType::Integer, &z, None).unwrap();

        let mut stmt = match stmt.execute() {
            ReturnOption::Success(s) | ReturnOption::Info(s) => s.close_cursor().unwrap(),
            ReturnOption::Error(s) | ReturnOption::NoData(s) => {
                panic!("Error executing INSERT: {}", get_last_error(&s));
            },
        };

        for i in 1..128 {
            *x.borrow_mut() = i;
            *y.borrow_mut() = 2*i;
            *z.borrow_mut() = 3*i;

            stmt = match stmt.execute() {
                ReturnOption::Success(s) | ReturnOption::Info(s) => s.close_cursor().unwrap(),
                ReturnOption::Error(s) | ReturnOption::NoData(s) => {
                    panic!("Error executing INSERT: {}", get_last_error(&s));
                },
            };
        }
    }
    {
        let mut rows: usize = 0;
        let x: RefCell<i32> = RefCell::new(0);
        let y: RefCell<i32> = RefCell::new(0);
        let z: RefCell<i32> = RefCell::new(0);

        let stmt = Statement::with_parent(&dbc).unwrap();
        let stmt = stmt.bind_col(1, &x, None).unwrap();
        let stmt = stmt.bind_col(2, &y, None).unwrap();
        let stmt = stmt.bind_col(3, &z, None).unwrap();
        let stmt = match stmt.exec_direct("SELECT x,y,z FROM tbl") {
            ReturnOption::Success(s) |
            ReturnOption::Info(s) => {
                assert_no_diagnostic(&s);
                s
            }
            ReturnOption::NoData(_) => panic!("No Data"),
            ReturnOption::Error(s) => panic!("{}", get_last_error(&s)),
        };
        assert_eq!(3, stmt.num_result_cols().unwrap());
        let mut stmt = match stmt.fetch() {
            ReturnOption::Success(s) | ReturnOption::Info(s) => s,
            ReturnOption::Error(s) => panic!("Error during fetching row: {}", get_last_error(&s)),
            ReturnOption::NoData(_) => panic!("Empty result set returned from SELECT"),
        };
        loop {
            rows += 1;
            let x = *x.borrow();
            let y = *y.borrow();
            let z = *z.borrow();
            assert_eq!( 2 * x, y );
            assert_eq!( 3 * x, z );

            stmt = match stmt.fetch() {
                ReturnOption::Success(s) | ReturnOption::Info(s) => s,
                ReturnOption::Error(s) => panic!("Error during fetching row: {}", get_last_error(&s)),
                ReturnOption::NoData(_) => break,
            };
        }
        assert_eq!( 128, rows );
    }
    dbc.disconnect().unwrap();
}

/// Checks for a diagnstic record. Should one be present this function panics printing the contents
/// of said record.
fn assert_no_diagnostic(diag: &Diagnostics) {
    use std::str;
    let mut buffer = [0; 512];
    match diag.diagnostics(1, &mut buffer) {
        ReturnOption::Success(dr) |
        ReturnOption::Info(dr) => {
            panic!(
                "{}",
                str::from_utf8(&buffer[0..(dr.text_length as usize)]).unwrap()
            )
        }
        ReturnOption::Error(()) => panic!("Error during fetching diagnostic record"),
        ReturnOption::NoData(()) => (),
    }
}

fn get_last_error(diag: &Diagnostics) -> String {
    use std::str;
    let mut buffer = [0; 512];
    match diag.diagnostics(1, &mut buffer) {
        ReturnOption::Success(dr) |
        ReturnOption::Info(dr) => {
            str::from_utf8(&buffer[0..(dr.text_length as usize)])
                .unwrap()
                .to_owned()
        }
        ReturnOption::Error(()) => panic!("Error during fetching diagnostic record"),
        ReturnOption::NoData(()) => panic!("No diagnostic available"),
    }
}
