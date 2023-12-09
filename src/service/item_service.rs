use crate::models::item::Item;
use crate::repository::item_repository::ItemRepository;
use sqlx::Error;

#[derive(Clone)]
pub struct ItemService {
    item_repo: ItemRepository,
}

impl ItemService {
    pub fn new(item_repo: ItemRepository) -> Self {
        Self { item_repo }
    }

    pub async fn get_item(&self, id: i32) -> Result<Option<Item>, Error> {
        self.item_repo.get_item(id).await
    }
}
