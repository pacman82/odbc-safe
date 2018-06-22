//! Transaction handling example
extern crate odbc_safe;
use odbc_safe::*;

fn main() {
    let env = Environment::new().unwrap();
    let env = env.declare_version_3().unwrap();

    let ds = DataSource::with_parent(&env).unwrap();
    let mut conn = ds.connect("TestDataSource", "", "").unwrap();

    //Set autocommit mode to false to allow manual transaction handling
    conn.set_autocommit(false).unwrap();

    //We could check if autocommit is on or off
    println!("Autocommit mode: {}", conn.is_autocommit());

    //Any statement now will start transaction which could be ended with conn.commit() or conn.rollback()
    //If either commit or rollback was not called before connection drop automatic rollback will be issued

    conn.commit().unwrap();
}
