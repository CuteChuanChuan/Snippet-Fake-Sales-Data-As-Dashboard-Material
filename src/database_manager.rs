use crate::data_generator::{Cities, Customer, ItemMaster, Sales, SalesRep};
use sqlx::{PgPool, Pool, Postgres};

pub struct DatabaseManager {
    pool: Pool<Postgres>,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn insert_cities(&self, cities: &Vec<Cities>) -> Result<(), sqlx::Error> {
        for city in cities {
            sqlx::query(
                r#"
                INSERT INTO public.cities (city_code, city, region, latitude, longitude, "desc")
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(&city.city_code)
            .bind(&city.city)
            .bind(&city.region)
            .bind(city.latitude)
            .bind(city.longitude)
            .bind(&city.desc)
            .execute(&self.pool)
            .await?;
        }
        println!("Already inserted {} rows of city data", cities.len());
        Ok(())
    }

    pub async fn insert_sales_reps(&self, sales_reps: &Vec<SalesRep>) -> Result<(), sqlx::Error> {
        for rep in sales_reps {
            sqlx::query(
                r#"
                INSERT INTO public.sales_reps (sales_rep_number, manager, manager_number, path,
                                       sales_rep_name_1, sales_rep_name_2, sales_rep_name_3)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            )
            .bind(rep.sales_rep_number)
            .bind(&rep.manager)
            .bind(rep.manager_number)
            .bind(&rep.path)
            .bind(&rep.sales_rep_name_1)
            .bind(&rep.sales_rep_name_2)
            .bind(&rep.sales_rep_name_3)
            .execute(&self.pool)
            .await?;
        }
        println!(
            "Already inserted {} rows of sales reps data",
            sales_reps.len()
        );
        Ok(())
    }

    pub async fn insert_item_master(
        &self,
        item_master: &Vec<ItemMaster>,
    ) -> Result<(), sqlx::Error> {
        for item in item_master {
            sqlx::query(
                r#"
                INSERT INTO public.item_masters (item_number, product_group, product_line,
                                        product_sub_group, product_type)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            )
            .bind(&item.item_number)
            .bind(&item.product_group)
            .bind(&item.product_line)
            .bind(&item.product_sub_group)
            .bind(&item.product_type)
            .execute(&self.pool)
            .await?;
        }
        println!(
            "Already inserted {} rows of item master data",
            item_master.len()
        );
        Ok(())
    }

    pub async fn insert_customers(&self, customers: &Vec<Customer>) -> Result<(), sqlx::Error> {
        for customer in customers {
            sqlx::query(
                r#"
                INSERT INTO public.customers (customer_number, customer, city_code)
                VALUES ($1, $2, $3)
            "#,
            )
            .bind(customer.customer_number)
            .bind(&customer.customer)
            .bind(&customer.city_code)
            .execute(&self.pool)
            .await?;
        }
        println!("Already inserted {} rows of customer data", customers.len());
        Ok(())
    }

    pub async fn insert_sales(&self, sales: &Vec<Sales>) -> Result<(), sqlx::Error> {
        for sale in sales {
            sqlx::query(
                r#"
                INSERT INTO public.sales ("key", cost, customer_number, date, gross_sales,
                                 invoice_date, invoice_number, item_desc, item_number,
                                 margin, order_number, promised_delivery_date, sales,
                                 sales_qty, sales_rep_number)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            "#,
            )
            .bind(&sale.key)
            .bind(sale.cost)
            .bind(sale.customer_number)
            .bind(sale.date)
            .bind(sale.gross_sales)
            .bind(sale.invoice_date)
            .bind(&sale.invoice_number)
            .bind(&sale.item_desc)
            .bind(&sale.item_number)
            .bind(sale.margin)
            .bind(&sale.order_number)
            .bind(sale.promised_delivery_date)
            .bind(sale.sales)
            .bind(sale.sales_qty)
            .bind(sale.sales_rep_number)
            .execute(&self.pool)
            .await?;
        }
        println!("Already inserted {} rows of sales data", sales.len());
        Ok(())
    }
}
