//! Transaction handling example
extern crate odbc_safe;
use odbc_safe::*;

fn main() {
    let env = Environment::new().unwrap();
    let env = env.declare_version_3().unwrap();

    let ds = DataSource::with_parent(&env).unwrap();
    let conn = ds.connect("TestDataSource", "", "").unwrap();
    let mut conn = conn.disable_autocommit().unwrap();

    {
        //Any statement now will start transaction which could be ended with conn.commit() or conn.rollback()
        //If either commit or rollback was not called before connection drop automatic rollback will be issued
        let stmt = Statement::with_parent(&conn).unwrap();
        let res = stmt.exec_direct("SELECT 'HELLO' FROM MOVIES");
        println!("Result {:?}", res);
    }

    let end_tx_result = conn.commit();

    println!("End TX result {:?}", end_tx_result);
}
