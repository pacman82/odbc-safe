//! Shows affected row count

extern crate odbc_safe;
use odbc_safe::*;

fn main() {
    let env = Environment::new().unwrap();
    let env = env.declare_version_3().unwrap();
    let conn = DataSource::with_parent(&env).unwrap();
    let conn = conn.connect("TestDataSource", "", "").unwrap();
    exec(&conn, "INSERT INTO movies (title, year) VALUES ('TEST movie', 9999), ('TEST movie', 9998)");
    exec(&conn, "DELETE FROM movies WHERE title = 'TEST movie'");
}

fn exec(conn: &Connection<AutocommitOn>, sql: &str) {
    let stmt = Statement::with_parent(conn).unwrap();
    let rs = match stmt.exec_direct(sql) {
        ReturnOption::Success(s) |
        ReturnOption::Info(s) => Ok(s),
        ReturnOption::NoData(_) => Err("Statement did not return a Result Set.".to_owned()),
        ReturnOption::Error(_) => Err("Error".to_owned()),
    };

    let row_count = rs.unwrap().affected_row_count();
    println!("Affected row count for last statement: {:?}", row_count);
}
