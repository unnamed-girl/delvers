use chronobase::DirectConnection;
use delver_sim::entities::{BaseCharacter, BaseTeam};

const DATABASE_PATH:&str = "DelverBase.db";

#[rocket::main()]
async fn main () {
    let connection = DirectConnection::new(DATABASE_PATH.to_string());
    connection.wipe().unwrap();
    chronobase::webserver::build(connection)
        .add_table::<BaseCharacter>()
        .add_patch_endpoint::<BaseCharacter>()
        .add_table::<BaseTeam>()
        .run_server().await
}