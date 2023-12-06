use crate::models::item::Item;
use sqlx::Error;
use crate::repository::itemRepo::ItemRepo;

#[derive(Clone)]
pub struct ItemService {
    item_repo: ItemRepo,
}

impl ItemService {
    pub fn new(item_repo: ItemRepo) -> Self {
        Self { item_repo }
    }

    pub async fn get_item(&self, id: i32) -> Result<Option<Item>, Error> {
        self.item_repo.get_item(id).await
    }
}
