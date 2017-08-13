extern crate odbc_safe;
use odbc_safe::*;
use std::str::from_utf8;

struct LastError(String);
type MyResult<T> = Result<T, LastError>;

impl<D: Diagnostics> From<D> for LastError {
    fn from(source: D) -> Self {
        let mut buffer = [0; 512];
        match source.diagnostics(1, &mut buffer) {
            ReturnOption::Success(dr) | ReturnOption::Info(dr) => LastError(
                from_utf8(&buffer[0..(dr.text_length as usize)])
                    .unwrap()
                    .to_owned(),
            ),
            ReturnOption::Error(()) => panic!("Error during fetching diagnostic record"),
            ReturnOption::NoData(()) => panic!("No diagnostic record present"),
        }
    }
}

fn main() {

    let env = Environment::new().unwrap();
    let env = env.declare_version_3().unwrap();

    match run(&env) {
        Ok(()) => (),
        Err(LastError(message)) => println!("An error occurred: {}", message),
    }
}

fn run(env: &Environment<Odbc3>) -> MyResult<()> {

    let conn = connect(&env);
    print_fields(execute_query(&conn)?)?;
    conn.disconnect().success().map(|_| ())
}

fn connect<V>(env: &Environment<V>) -> Connection<Connected>
where
    V: Version,
{
    let conn = Connection::with_parent(env).unwrap();
    conn.connect("TestDataSource", "", "").unwrap()
}

fn execute_query<'a, 'b>(conn: &'a Connection<Connected>) -> MyResult<Statement<'a, 'b, Opened>> {
    let stmt = Statement::with_parent(conn).unwrap();
    match stmt.exec_direct("SELECT * FROM MOVIES") {
        ReturnOption::Success(s) | ReturnOption::Info(s) => Ok(s),
        ReturnOption::NoData(_) => Err(LastError(
            "Statement did not return a Result Set.".to_owned(),
        )),
        ReturnOption::Error(e) => Err(e.into()),
    }
}

fn print_fields(result_set: Statement<Opened>) -> MyResult<()> {
    let cols = result_set.num_result_cols().unwrap();
    let mut buffer = [0u8; 512];
    let mut cursor = match result_set.fetch() {
        ReturnOption::Success(r) | ReturnOption::Info(r) => r,
        ReturnOption::NoData(_) => return Ok(()),
        ReturnOption::Error(e) => return Err(e.into()),
    };
    loop {
        for index in 1..(cols + 1) {
            match cursor.get_data(index as u16, &mut buffer as &mut [u8]) {
                ReturnOption::Success(ind) | ReturnOption::Info(ind) => match ind {
                    Indicator::NoTotal => panic!("No Total"),
                    Indicator::Null => println!("NULL"),
                    Indicator::Length(l) => {
                        print!("{}", from_utf8(&buffer[0..l as usize]).unwrap());
                    }
                },
                ReturnOption::NoData(_) => panic!("No Field Data"),
                ReturnOption::Error(_) => return Err(cursor.into()),
            }
            print!(" | ");
        }
        cursor = match cursor.fetch() {
            ReturnOption::Success(r) | ReturnOption::Info(r) => r,
            ReturnOption::NoData(_) => break Ok(()),
            ReturnOption::Error(e) => break Err(e.into()),
        };
        println!("");
    }
}
