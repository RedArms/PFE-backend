use crate::models::item::Item;
use crate::models::user::User;
use crate::ucc::pgsqlConn::pgsqlConn;


pub async fn get_user(id:i32) ->  Option<User> {

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let db_conn = match pgsqlConn::new(&database_url).await {
        Ok(conn) => conn,
        Err(err) => {
            println!("{}", err);
            return None},
    };

    let user = db_conn.get_user(id as i32).await.unwrap();

    return user;

}