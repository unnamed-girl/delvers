use chronobase::DirectConnection;
use delver_sim::entities::{Character, Team};

const DATABASE_PATH:&str = "DelverBase.db";

#[rocket::main()]
async fn main () {
    let connection = DirectConnection::new(DATABASE_PATH.to_string());
    connection.wipe().unwrap();
    chronobase::webserver::build(connection)
    .add_table::<Character>()
    .add_table::<Team>()
    .run_server().await
}