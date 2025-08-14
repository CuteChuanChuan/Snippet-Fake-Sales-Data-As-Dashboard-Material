use chrono::NaiveDate;
use rand::Rng;
use rand::prelude::ThreadRng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesRep {
    pub sales_rep_number: i32,
    pub manager: String,
    pub manager_number: i32,
    pub path: String,
    pub sales_rep_name_1: String,
    pub sales_rep_name_2: String,
    pub sales_rep_name_3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub customer_number: i32,
    pub customer: String,
    pub city_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cities {
    pub city_code: String,
    pub city: String,
    pub region: String,
    pub latitude: f64,
    pub longitude: f64,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMaster {
    pub item_number: String,
    pub product_group: String,
    pub product_line: String,
    pub product_sub_group: String,
    pub product_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sales {
    pub key: String,
    pub cost: f64,
    pub customer_number: i32,
    pub date: NaiveDate,
    pub gross_sales: f64,
    pub invoice_date: NaiveDate,
    pub invoice_number: String,
    pub item_desc: String,
    pub item_number: String,
    pub margin: f64,
    pub order_number: String,
    pub promised_delivery_date: NaiveDate,
    pub sales: f64,
    pub sales_qty: i32,
    pub sales_rep_number: i32,
}

pub struct DataGenerator {
    rng: ThreadRng,
    city_names: Arc<Vec<&'static str>>,
    city_codes: Arc<Vec<&'static str>>,
    regions: Arc<Vec<&'static str>>,
    first_names: Arc<Vec<&'static str>>,
    last_names: Arc<Vec<&'static str>>,
    managers: Arc<Vec<&'static str>>,
    company_types: Arc<Vec<&'static str>>,
    company_names: Arc<Vec<&'static str>>,
    product_groups: Arc<Vec<&'static str>>,
    product_lines: Arc<Vec<&'static str>>,
    product_types: Arc<Vec<&'static str>>,

    cities: Vec<Cities>,
    customers: Vec<Customer>,
    sales_reps: Vec<SalesRep>,
    item_master: Vec<ItemMaster>,
}

impl DataGenerator {
    pub fn new() -> Self {
        Self {
            rng: rand::rng(),
            city_names: Arc::new(vec![
                "Taipei",
                "Kaohsiung",
                "Taichung",
                "Tainan",
                "Hsinchu",
                "Keelung",
                "Chiayi",
                "Changhua",
                "Taoyuan",
                "Hualien",
                "Taitung",
                "Pingtung",
                "Yilan",
                "Nantou",
                "Miaoli",
                "Yunlin",
            ]),
            city_codes: Arc::new(vec![
                "C01", "C02", "C03", "C04", "C05", "C06", "C07", "C08", "C09", "C10", "C11", "C12",
                "C13", "C14", "C15", "C16",
            ]),
            regions: Arc::new(vec!["North", "Central", "South", "East"]),
            first_names: Arc::new(vec![
                "John", "Alice", "Bob", "Carol", "David", "Emma", "Frank", "Grace",
            ]),
            last_names: Arc::new(vec![
                "Smith", "Johnson", "Brown", "Davis", "Miller", "Wilson", "Moore", "Taylor",
            ]),
            managers: Arc::new(vec!["Manager A", "Manager B", "Manager C", "Manager D"]),
            company_types: Arc::new(vec!["Corp", "Ltd", "Inc", "Co", "LLC"]),
            company_names: Arc::new(vec![
                "TechFlow",
                "DataSync",
                "CloudMax",
                "InfoTech",
                "SystemPro",
                "DigitalHub",
                "NetCore",
                "CodeBase",
                "WebFlow",
                "AppTech",
            ]),
            product_groups: Arc::new(vec!["Electronics", "Software", "Hardware", "Services"]),
            product_lines: Arc::new(vec!["Premium", "Standard", "Basic", "Enterprise"]),
            product_types: Arc::new(vec!["Physical", "Digital", "Service", "Subscription"]),

            cities: Vec::new(),
            customers: Vec::new(),
            sales_reps: Vec::new(),
            item_master: Vec::new(),
        }
    }

    pub fn generate_cities(&mut self, count: usize) -> &Vec<Cities> {
        let mut cities = Vec::with_capacity(count);

        for i in 0..count {
            let city_name = self.city_names[i % self.city_names.len()];
            let city_code = self.city_codes[i % self.city_codes.len()];
            let region = self.regions[self.rng.random_range(0..self.regions.len())];

            cities.push(Cities {
                city_code: city_code.to_string(),
                city: city_name.to_string(),
                region: region.to_string(),
                latitude: self.rng.random_range(22.0..25.5),
                longitude: self.rng.random_range(120.0..122.0),
                desc: city_name.to_string(),
            });
        }

        self.cities = cities;

        &self.cities
    }

    pub fn generate_sales_reps(&mut self, count: usize) -> &Vec<SalesRep> {
        let mut sales_reps = Vec::with_capacity(count);

        for i in 0..count {
            let first_name = self.first_names[self.rng.random_range(0..self.first_names.len())];
            let last_name = self.last_names[self.rng.random_range(0..self.last_names.len())];
            let manager_idx = self.rng.random_range(0..self.managers.len());
            let manager = self.managers[manager_idx];

            sales_reps.push(SalesRep {
                sales_rep_number: i as i32 + 1000,
                manager: manager.to_string(),
                manager_number: manager_idx as i32,
                path: format!("Path_{}", i + 1),
                sales_rep_name_1: first_name.to_string(),
                sales_rep_name_2: last_name.to_string(),
                sales_rep_name_3: format!("{} {}", first_name, last_name),
            });
        }

        self.sales_reps = sales_reps;

        &self.sales_reps
    }

    pub fn generate_customers(&mut self, count: usize) -> &Vec<Customer> {
        if self.cities.is_empty() {
            self.generate_cities(16);
        }

        let mut customers = Vec::with_capacity(count);

        for i in 0..count {
            let company_type =
                self.company_types[self.rng.random_range(0..self.company_types.len())];
            let company_name =
                self.company_names[self.rng.random_range(0..self.company_names.len())];

            let city_code = &self.cities[self.rng.random_range(0..self.cities.len())].city_code;

            customers.push(Customer {
                customer_number: i as i32 + 10000,
                customer: format!("{} {}", company_name, company_type),
                city_code: city_code.clone(),
            });
        }

        self.customers = customers;

        &self.customers
    }

    pub fn generate_item_master(&mut self, count: usize) -> &Vec<ItemMaster> {
        let mut item_masters = Vec::with_capacity(count);

        for i in 0..count {
            let group = self.product_groups[self.rng.random_range(0..self.product_groups.len())];
            let line = self.product_lines[self.rng.random_range(0..self.product_lines.len())];
            let p_type = self.product_types[self.rng.random_range(0..self.product_types.len())];

            item_masters.push(ItemMaster {
                item_number: format!("ITM{:06}", i + 1),
                product_group: group.to_string(),
                product_line: line.to_string(),
                product_sub_group: format!("{}_Sub", group),
                product_type: p_type.to_string(),
            });
        }

        self.item_master = item_masters;

        &self.item_master
    }

    pub fn generate_sales(&mut self, count: usize) -> Vec<Sales> {
        if self.customers.is_empty() {
            self.generate_customers(800);
        }
        if self.sales_reps.is_empty() {
            self.generate_sales_reps(100);
        }
        if self.item_master.is_empty() {
            self.generate_item_master(100);
        }

        let mut sales = Vec::with_capacity(count);
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        for i in 0..count {
            let customer = &self.customers[self.rng.random_range(0..self.customers.len())];
            let sales_rep = &self.sales_reps[self.rng.random_range(0..self.sales_reps.len())];
            let item_master = &self.item_master[self.rng.random_range(0..self.item_master.len())];
            let days_offset = self.rng.random_range(0..365);
            let date = start_date + chrono::Duration::days(days_offset as i64);

            let cost = self.rng.random_range(100.0..5000.0);
            let margin = self.rng.random_range(0.2..0.5);
            let gross_sales = cost / (1.0 - margin);
            let sales_amount = gross_sales * 0.9;

            sales.push(Sales {
                key: format!("SALE{:08}", i + 1),
                cost,
                customer_number: customer.customer_number,
                date,
                gross_sales,
                invoice_date: date + chrono::Duration::days(1),
                invoice_number: format!("INV{:08}", i + 1),
                item_desc: format!(
                    "{} - {}",
                    item_master.product_group, item_master.product_line
                ),
                item_number: item_master.item_number.clone(),
                margin,
                order_number: format!("ORD{:08}", i + 1),
                promised_delivery_date: date + chrono::Duration::days(7),
                sales: sales_amount,
                sales_qty: self.rng.random_range(1..100),
                sales_rep_number: sales_rep.sales_rep_number,
            });
        }

        sales
    }

    pub fn get_cities(&self) -> &Vec<Cities> {
        &self.cities
    }

    pub fn get_sales_reps(&self) -> &Vec<SalesRep> {
        &self.sales_reps
    }

    pub fn get_customers(&self) -> &Vec<Customer> {
        &self.customers
    }

    pub fn get_item_master(&self) -> &Vec<ItemMaster> {
        &self.item_master
    }
}
