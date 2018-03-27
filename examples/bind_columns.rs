//! Binds columns to a result set and fills them with fetch. This is still super akward.
extern crate odbc_safe;
extern crate odbc_sys;
use odbc_safe::*;
use odbc_sys::SQLLEN;
use std::str::from_utf8;

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

fn execute_query<'a>(conn: &'a Connection) -> MyResult<ResultSet<'a, 'a, 'a, Unprepared>> {
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

fn print_fields(result_set: ResultSet<Unprepared>) -> MyResult<()> {
    let mut year = 0;
    let mut title = [0u8; 512];
    let mut ind_year = 0;
    let mut ind_title = 0;
    let mut cursor_opt = fetch(
        result_set,
        &mut year,
        &mut title,
        &mut ind_year,
        &mut ind_title,
    )?;
    while let Some(p) = cursor_opt {
        println!(
            "year: {}, title: {}",
            year,
            from_utf8(&title[0..(ind_title as usize)]).unwrap()
        );
        cursor_opt = fetch(p, &mut year, &mut title, &mut ind_year, &mut ind_title)?
    }
    Ok(())
}

fn fetch<'con, 'p, 'c, C>(
    cursor: Statement<'con, 'p, 'c, C>,
    year: &mut u32,
    title: &mut [u8],
    ind_year: &mut SQLLEN,
    ind_title: &mut SQLLEN,
) -> MyResult<Option<Statement<'con, 'p, 'c, Positioned>>>
where
    C: CursorState,
{
    use ReturnOption::*;
    let cursor = cursor.bind_col(1, year, Some(ind_year)).into_result()?;
    let cursor = cursor.bind_col(2, &mut title[..], Some(ind_title)).into_result()?;
    let cursor = match cursor.fetch() {
        Success(s) | Info(s) => Some(s.reset_columns()),
        NoData(_) => None,
        Error(s) => return Err(s.into()),
    };
    Ok(cursor)
}
