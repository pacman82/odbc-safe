//! Prepares an SQL query and executes it once for every parameter in an array
extern crate odbc_safe;
use odbc_safe::*;
use std::str::from_utf8;

fn main() {

    let env = Environment::new().unwrap();
    let env = env.declare_version_3().unwrap();
    let conn = connect(&env);
    let mut stmt = prepare_query(&conn);
    for &year in [1968, 1993].iter() {
        let result_set = execute_query(stmt, year);
        stmt = print_fields(result_set);
        println!("");
    }
}

fn connect<V>(env: &Environment<V>) -> Connection
where
    V: Version,
{
    let conn = DataSource::with_parent(env).unwrap();
    conn.connect("TestDataSource", "", "").unwrap()
}

fn prepare_query<'a>(conn: &'a Connection) -> Statement<'a, 'a, 'a, NoCursor, Prepared> {
    let stmt = Statement::with_parent(conn).unwrap();
    stmt.prepare("SELECT TITLE FROM MOVIES WHERE YEAR = ?")
        .unwrap()
}

fn execute_query<'a>(
    stmt: Statement<'a, 'a, 'a, NoCursor, Prepared>,
    year: i32,
) -> ResultSet<'a, 'a, 'a, Prepared> {
    let stmt = stmt.bind_input_parameter(1, DataType::Integer, Some(&year))
        .unwrap();
    let stmt = match stmt.execute() {
        ReturnOption::Success(s) |
        ReturnOption::Info(s) => s,
        ReturnOption::NoData(_) |
        ReturnOption::Error(_) => panic!("No Result Set"),
    };
    stmt.reset_parameters()
}

fn print_fields<'a>(
    result_set: ResultSet<'a, 'a, 'a, Prepared>,
) -> Statement<'a, 'a, 'a, NoCursor, Prepared> {
    let mut buffer = [0u8; 512];
    let mut cursor = match result_set.fetch() {
        ReturnOption::Success(r) |
        ReturnOption::Info(r) => r,
        ReturnOption::NoData(r) |
        ReturnOption::Error(r) => return r,
    };
    loop {
        match cursor.get_data(1, &mut buffer as &mut [u8]) {
            ReturnOption::Success(ind) |
            ReturnOption::Info(ind) => {
                match ind {
                    Indicator::NoTotal => panic!("No Total"),
                    Indicator::Null => println!("NULL"),
                    Indicator::Length(l) => {
                        print!("{}", from_utf8(&buffer[0..l as usize]).unwrap());
                    }
                }
            }
            ReturnOption::NoData(_) |
            ReturnOption::Error(_) => panic!("No Field Data"),
        }
        cursor = match cursor.fetch() {
            ReturnOption::Success(r) |
            ReturnOption::Info(r) => r,
            ReturnOption::NoData(r) |
            ReturnOption::Error(r) => break r,
        };
        println!("");
    }
}
