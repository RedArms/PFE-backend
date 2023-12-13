use crate::models::quantity_left_model::QuantityLeftModel;
use crate::models::tours::Tours;
use crate::models::tours_day::ToursDay;
use actix_web::web;
use chrono::NaiveDate;
use sqlx::Error;

#[derive(Clone)]
pub struct ToursRepository {
    app_state: web::Data<crate::AppState>,
}

impl ToursRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_all_tours(&self) -> Result<Vec<Tours>, Error> {
        let tours = sqlx::query_as!(Tours, "SELECT * FROM pfe.tours")
            .fetch_all(&self.app_state.db_pool)
            .await?;

        Ok(tours)
    }

    pub async fn get_all_tours_day(&self) -> Result<Vec<ToursDay>, Error> {
        let tours = sqlx::query_as!(ToursDay, "SELECT * FROM pfe.tour_days")
            .fetch_all(&self.app_state.db_pool)
            .await?;
        Ok(tours)
    }

    pub async fn get_tours_deliverer_day(&self, deliverer: i32) -> Result<ToursDay, Error> {
        let tour = sqlx::query_as!(
            ToursDay,
            "SELECT * FROM pfe.tour_days WHERE delivery_person = $1",
            deliverer
        )
        .fetch_one(&self.app_state.db_pool)
        .await?;

        Ok(tour)
    }

    pub async fn get_tours_today(&self) -> Result<Vec<ToursDay>, Error> {
        let current_date: chrono::prelude::NaiveDate = chrono::Local::now().naive_local().date();
        let tours = sqlx::query_as!(
            ToursDay,
            "SELECT * FROM pfe.tour_days WHERE date = $1",
            current_date
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        Ok(tours)
    }

    pub async fn get_tours_ay(&self) -> Result<Vec<ToursDay>, Error> {
        let current_date: chrono::prelude::NaiveDate = chrono::Local::now().naive_local().date();
        let tours = sqlx::query_as!(
            ToursDay,
            "SELECT * FROM pfe.tour_days WHERE date = $1",
            current_date
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        Ok(tours)
    }

    pub async fn set_deliverer(
        &self,
        date: String,
        tour: i32,
        deliverer_id: i32,
    ) -> Result<u64, Error> {
        let current_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
        let result = sqlx::query!(
            "UPDATE pfe.tour_days SET delivery_person  = $1 WHERE tour = $2 AND date = $3 ",
            deliverer_id,
            tour,
            current_date
        )
        .execute(&self.app_state.db_pool)
        .await?;

        println!("result: {:?}", result.rows_affected());
        Ok(result.rows_affected())
    }
    pub async fn get_tours_day_avalaible(&self) -> Result<Vec<ToursDay>, Error> {
        let current_date: chrono::prelude::NaiveDate = chrono::Local::now().naive_local().date();
        let tours = sqlx::query_as!(
            ToursDay,
            "SELECT * FROM pfe.tour_days WHERE date = $1 AND delivery_person IS NULL",
            current_date,
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        Ok(tours)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Tours>, Error> {
        let tour = sqlx::query_as!(Tours, "SELECT * FROM pfe.tours WHERE tour_id= $1", id)
            .fetch_optional(&self.app_state.db_pool)
            .await?;

        println!("tour: {:?}", tour);
        match tour {
            Some(tour) => Ok(Some(tour)),
            None => Ok(None),
        }
    }
    pub async fn get_tours_by_delivery_day(&self, date: String) -> Result<Vec<ToursDay>, Error> {
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
        let tours = sqlx::query_as!(
            ToursDay,
            "SELECT * FROM pfe.tour_days WHERE date = $1 ",
            date
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        Ok(tours)
    }

    pub async fn get_tours_for_deliverer(
        &self,
        deliverer_id: i32,
    ) -> Result<Option<ToursDay>, Error> {
        let current_date: chrono::prelude::NaiveDate = chrono::Local::now().naive_local().date();
        let tour = sqlx::query_as!(
            ToursDay,
            "SELECT * FROM pfe.tour_days WHERE delivery_person = $1 AND  date = $2",
            deliverer_id,
            current_date
        )
        .fetch_optional(&self.app_state.db_pool)
        .await?;

        Ok(tour)
    }

    pub async fn get_quatity_left(
        &self,
        date: String,
        tour: i32,
    ) -> Result<Vec<QuantityLeftModel>, Error> {
        let date_parse = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
        let quantity_left = sqlx::query_as!(
            QuantityLeftModel,
            "SELECT td.tour, td.date, i.label, i.size, SUM(b.quantity - b.delivered_qty) AS quantity
                FROM pfe.tour_days td
                JOIN pfe.orders o ON td.tour = o.tour AND td.date = o.date
                JOIN pfe.boxes b ON o.order_id = b.order_id
                JOIN pfe.items i ON b.item = i.item_id
                WHERE td.date = $1 AND td.tour = $2
                GROUP BY td.tour, td.date, b.item, i.label, i.size;",
            date_parse,
            tour
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;
        println!("quantity_left: {:?}", quantity_left);
        Ok(quantity_left)
    }
}
