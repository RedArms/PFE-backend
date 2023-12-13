    use crate::AppState;
    use chrono::{Datelike, Duration, Local, Utc};
    use sqlx::error::Error as SqlxError;
    use sqlx::{postgres::PgPool};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use tokio::time::sleep;
    use tokio::signal::ctrl_c;


    // Asynchronous function to create a "tour day" in the database
    async fn create_tour_day(db_pool: &PgPool) -> Result<(), sqlx::Error> {
        // Display a debug message
        println!("Creating tour day");

        // Get the current date in UTC
        let current_date = Utc::now();

        // Calculate a new date three days from the current date
        let new_date = current_date + Duration::days(3);

        // Check if the new date falls on a weekend (Saturday or Sunday)
        if new_date.weekday().number_from_monday() > 5 {
            // Return an error if it's a weekend
            return Err(SqlxError::Io(*Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Weekend: no tour day created",
            ))));
        }

        // Format the date for the database
        let formatted_date =
            chrono::NaiveDate::parse_from_str(&new_date.format("%Y-%m-%d").to_string(), "%Y-%m-%d")
                .unwrap();

        // Call a stored SQL procedure to create the "tour day"
        sqlx::query!("call create_tour_day( $1 );", formatted_date)
            .execute(db_pool)
            .await?;

        // Return a successful result
        Ok(())
    }

    // Asynchronous function to run the daily automation
    pub async fn run_daily_automation(app_state: AppState) {
        // Create an atomic boolean for managing execution state
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        // Handle Ctrl+C signal for graceful shutdown
        tokio::spawn(async move {
            ctrl_c().await.expect("Failed to listen for Ctrl+C");
            println!("Received Ctrl+C, shutting down gracefully...");
            running_clone.store(false, Ordering::Relaxed);
        });

        // Clone the database connection pool from the application state
        let db_pool = app_state.db_pool.clone();

        // Main loop for daily automation
        while running.load(Ordering::Relaxed) {
            // Get the current local time
            let now = Local::now();

            // Calculate the next day at 00:00:00
            let next_day = now.date().succ_opt().unwrap_or(now.date()).and_hms(0, 0, 0);

            // Calculate the duration until the next day
            let duration_until_next_day = next_day.signed_duration_since(now);

            // Call the function to create the "tour day" and wait until the next day
            let create_run = create_tour_day(&db_pool).await;

            sleep(tokio::time::Duration::from_secs(
                duration_until_next_day.num_seconds() as u64,
            )).await;

            // Print the results of the "tour day" creation
            match create_run {
                Ok(()) => println!("Tuple créé avec succès"), // Tuple created successfully
                Err(e) => eprintln!("Erreur lors de la création du tuple : {:?}", e), // Error during tuple creation
            }
        }
    }