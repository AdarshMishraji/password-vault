use dotenvy::dotenv;

#[derive(Clone, Debug)]
pub struct Env {
    pub database_url: String,
}

pub fn new() -> Env {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    Env { database_url }
}
