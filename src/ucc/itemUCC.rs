use crate::models::item::Item;
use crate::ucc::pgsqlConn::pgsqlConn;


pub async fn get_item(id:i32) -> Option<Item> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let db_conn = match pgsqlConn::new(&database_url).await {
        Ok(conn) => conn,
        Err(err) => {
            println!("{}", err);
            return None},
    };

    let item = db_conn.get_item(id as i32).await.unwrap();

    return item;
}