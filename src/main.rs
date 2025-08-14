mod data_generator;
mod database_manager;

use crate::database_manager::DatabaseManager;
use data_generator::DataGenerator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = DatabaseManager::new(&database_url).await?;
    let mut data_generator = DataGenerator::new();
    let cities = data_generator.generate_cities(16);
    db.insert_cities(cities).await?;
    let sales_reps = data_generator.generate_sales_reps(5);
    db.insert_sales_reps(sales_reps).await?;

    let item_masters = data_generator.generate_item_master(50);
    db.insert_item_master(item_masters).await?;

    let customers = data_generator.generate_customers(10);
    db.insert_customers(customers).await?;

    let sales = data_generator.generate_sales(1000);
    db.insert_sales(&sales).await?;

    println!("All done！");

    Ok(())
}
