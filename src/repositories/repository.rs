use async_trait::async_trait;
use tokio_postgres::{NoTls, Error, Client};

pub struct Db {}

#[async_trait]
pub trait Repository {
    async fn find_all() -> Result<Vec<Self>, Error> where Self: Sized;

    async fn find_by_id(id: String) -> Result<Option<Self>, Error> where Self: Sized;

    async fn find_by_name(name: String) -> Result<Option<Self>, Error> where Self: Sized;

    async fn save(&self) -> Result<(), Error>;
}

impl Db {
    pub async fn db_connect() -> Result<Client, Error> {
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres password=password dbname=ecommerce", NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprint!("connection error: {}", e);
            }
        });

        Ok(client)
    }

    pub async fn check_tables() -> Result<(), Error> {
        println!("Connecting to database.");

        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres password=password dbname=ecommerce", NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprint!("connection error: {}", e);
            }
        });

        println!("Checking if category table exists.");

        let _category = client
            .query("CREATE TABLE IF NOT EXISTS ecommerce.category (
                id VARCHAR NOT NULL,
                name VARCHAR NOT NULL,
                PRIMARY KEY (id)
            );", &[]).await?;

        println!("Checking if product table exists.");

        let _product = client
            .query("CREATE TABLE IF NOT EXISTS ecommerce.product (
                id VARCHAR NOT NULL,
                  name VARCHAR NOT NULL,
                  price FLOAT(53) NOT NULL,
                  quantity_in_stock INT NOT NULL,
                  description VARCHAR NOT NULL,
                  category_id VARCHAR NOT NULL REFERENCES ecommerce.category(id),
                  PRIMARY KEY (id)
            );", &[]).await?;

        println!("Database configuration complete.\r\n");
        Ok(())
    }
}