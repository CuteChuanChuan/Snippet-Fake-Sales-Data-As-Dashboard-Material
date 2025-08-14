-- Cities
create table if not exists postgres.public.cities
(
    city_code text primary key,
    city      text             not null,
    region    text             not null,
    latitude  double precision not null,
    longitude double precision not null,
    "desc"    text             not null
);

-- Sales Representatives
create table if not exists postgres.public.sales_reps
(
    sales_rep_number integer primary key,
    manager          text    not null,
    manager_number   integer not null,
    path             text    not null,
    sales_rep_name_1 text    not null,
    sales_rep_name_2 text    not null,
    sales_rep_name_3 text    not null
);

-- Item Master
create table if not exists postgres.public.item_masters
(
    item_number       text primary key,
    product_group     text not null,
    product_line      text not null,
    product_sub_group text not null,
    product_type      text not null
);

-- Customers
create table if not exists postgres.public.customers
(
    customer_number integer primary key,
    customer        text not null,
    city_code       text not null
);

-- Sales
create table if not exists postgres.public.sales
(
    "key"                  text primary key,
    cost                   double precision not null,
    customer_number        integer          not null,
    date                   date             not null,
    gross_sales            double precision not null,
    invoice_date           date             not null,
    invoice_number         text             not null,
    item_desc              text             not null,
    item_number            text             not null,
    margin                 double precision not null,
    order_number           text             not null,
    promised_delivery_date date             not null,
    sales                  double precision not null,
    sales_qty              integer          not null,
    sales_rep_number       integer          not null
);
