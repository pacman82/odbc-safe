//! Binds columns to a result set and fills them with fetch. This is still super akward.
extern crate odbc_safe;
extern crate odbc_sys;
use odbc_safe::*;
use odbc_sys::SQLLEN;
use std::str::from_utf8;
use std::cell::RefCell;

// Setup error handling
struct LastError(String);
type MyResult<T> = Result<T, LastError>;

impl<D: Diagnostics> From<D> for LastError {
    fn from(source: D) -> Self {
        let mut buffer = [0; 512];
        match source.diagnostics(1, &mut buffer) {
            ReturnOption::Success(dr) |
            ReturnOption::Info(dr) => LastError(
                from_utf8(&buffer[0..(dr.text_length as usize)])
                    .unwrap()
                    .to_owned(),
            ),
            ReturnOption::Error(()) => panic!("Error during fetching diagnostic record"),
            ReturnOption::NoData(()) => LastError("No Diagnostic Record present".to_owned()),
        }
    }
}

trait ExtReturn<T> {
    fn into_result(self) -> MyResult<T>;
}

impl<T, D> ExtReturn<T> for Return<T, D>
where
    D: Diagnostics,
{
    fn into_result(self) -> MyResult<T> {
        match self {
            Success(v) | Info(v) => Ok(v),
            Error(d) => Err(d.into()),
        }
    }
}

// Actual application
fn main() {

    let env = Environment::new().unwrap();
    let env = env.declare_version_3().unwrap();

    match run(&env) {
        Ok(()) => (),
        Err(LastError(message)) => println!("An error occurred: {}", message),
    }
}

fn run(env: &Environment<Odbc3>) -> MyResult<()> {

    let conn = connect(env)?;
    let result_set = execute_query(&conn)?;
    print_fields(result_set)
}

fn connect<V>(env: &Environment<V>) -> MyResult<Connection>
where
    V: Version,
{
    let conn = DataSource::with_parent(env).unwrap();
    conn.connect("TestDataSource", "", "").into_result()
}

fn execute_query<'a>(conn: &'a Connection) -> MyResult<ResultSet<'a, (), (), Unprepared>> {
    let stmt = Statement::with_parent(conn).unwrap();
    match stmt.exec_direct("SELECT year, title FROM Movies") {
        ReturnOption::Success(s) |
        ReturnOption::Info(s) => Ok(s),
        ReturnOption::NoData(_) => Err(LastError(
            "Statement did not return a Result Set.".to_owned(),
        )),
        ReturnOption::Error(e) => Err(e.into()),
    }
}

fn print_fields(result_set: ResultSet<(), (), Unprepared>) -> MyResult<()> {
    let year = RefCell::new(0);
    let title = RefCell::new([0u8; 512]);
    let ind_year = RefCell::new(0);
    let ind_title = RefCell::new(0);
    let mut cursor_opt = fetch(
        result_set,
        &year,
        &title,
        &ind_year,
        &ind_title,
    )?;
    while let Some(p) = cursor_opt {
        println!(
            "year: {}, title: {}",
            year.borrow(),
            from_utf8(&title.borrow()[0..(*ind_title.borrow() as usize)]).unwrap()
        );
        cursor_opt = fetch(p, &year, &title, &ind_year, &ind_title)?
    }
    Ok(())
}

fn fetch<'con, C>(
    cursor: Statement<'con, (), (), C>,
    year: &RefCell<u32>,
    title: &RefCell<[u8; 512]>,
    ind_year: &RefCell<SQLLEN>,
    ind_title: &RefCell<SQLLEN>,
) -> MyResult<Option<Statement<'con, (), (), Positioned>>>
where
    C: CursorState,
{
    use ReturnOption::*;
    let cursor = cursor.bind_col(1, year, Some(ind_year)).into_result()?;
    let cursor = cursor.bind_col(2, title, Some(ind_title)).into_result()?;
    let cursor = match cursor.fetch() {
        Success(s) | Info(s) => Some(s.reset_columns()),
        NoData(_) => None,
        Error(s) => return Err(s.into()),
    };
    Ok(cursor)
}
