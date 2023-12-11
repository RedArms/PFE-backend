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

    pub async fn get_all_items(&self) -> Result<Vec<Item>, Error> {
        self.item_repo.get_all_items().await
    }

    pub async fn create_item(&self, item: Item) -> Result<Item, Error> {
        self.item_repo.create_item(item).await
    }
}
