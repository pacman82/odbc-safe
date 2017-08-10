extern crate odbc_safe;
use odbc_safe::*;
use std::str::from_utf8;

fn main() {

    let env = Environment::new().unwrap();
    let env = env.declare_version_3().unwrap();
    let conn = connect(&env);
    print_fields(execute_query(&conn));
    conn.disconnect().unwrap();
}

fn connect<V>(env: &Environment<V>) -> Connection<Connected>
where
    V: Version,
{
    let conn = Connection::with_parent(env).unwrap();
    conn.connect("TestDataSource", "", "").unwrap()
}

fn execute_query<'a>(conn: &'a Connection<Connected>) -> Statement<'a, HasResult> {
    let stmt = Statement::with_parent(conn).unwrap();
    match stmt.exec_direct("SELECT * FROM MOVIES") {
        ReturnNoData::Success(s) | ReturnNoData::Info(s) => s,
        ReturnNoData::NoData(_) | ReturnNoData::Error(_) => panic!("No Result Set"),
    }
}

fn print_fields(mut result_set: Statement<HasResult>) {
    let cols = result_set.num_result_cols().unwrap();
    let mut buffer = [0u8; 512];
    loop {
        result_set = match result_set.fetch() {
            ReturnNoData::Success(r) | ReturnNoData::Info(r) => r,
            ReturnNoData::NoData(_) | ReturnNoData::Error(_) => break,
        };
        for index in 1..(cols + 1) {
            match result_set.get_data(index as u16, &mut buffer as &mut [u8]) {
                ReturnNoData::Success(ind) | ReturnNoData::Info(ind) => {
                    match ind {
                        Indicator::NoTotal => panic!("No Total"),
                        Indicator::Null => println!("NULL"),
                        Indicator::Length(l) => {
                            print!("{}", from_utf8(&buffer[0..l as usize]).unwrap());
                        }
                    }
                }
                ReturnNoData::NoData(_) | ReturnNoData::Error(_) => panic!("No Field Data"),
            }
            print!(" | ");
        }
        println!("");
    }
}
