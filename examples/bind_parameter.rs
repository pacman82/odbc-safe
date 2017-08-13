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

fn execute_query<'a, 'b>(conn: &'a Connection<Connected>) -> Statement<'a, 'b, HasResult> {
    let stmt = Statement::with_parent(conn).unwrap();
    let param = 1968;
    let stmt = stmt.bind_input_parameter(1, DataType::Integer, Some(&param))
        .unwrap();
    let stmt = match stmt.exec_direct("SELECT * FROM MOVIES WHERE YEAR = ?;") {
        ReturnOption::Success(s) |
        ReturnOption::Info(s) => s,
        ReturnOption::NoData(_) |
        ReturnOption::Error(_) => panic!("No Result Set"),
    };
    stmt.reset_parameters()
}

fn print_fields(result_set: Statement<HasResult>) {
    let cols = result_set.num_result_cols().unwrap();
    let mut buffer = [0u8; 512];
    let mut cursor = match result_set.fetch() {
        ReturnOption::Success(r) |
        ReturnOption::Info(r) => r,
        ReturnOption::NoData(_) |
        ReturnOption::Error(_) => return,
    };
    loop {
        for index in 1..(cols + 1) {
            match cursor.get_data(index as u16, &mut buffer as &mut [u8]) {
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
            print!(" | ");
        }
        cursor = match cursor.fetch() {
            ReturnOption::Success(r) |
            ReturnOption::Info(r) => r,
            ReturnOption::NoData(_) |
            ReturnOption::Error(_) => break,
        };
        println!("");
    }
}
