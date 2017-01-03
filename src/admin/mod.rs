mod import;
mod config;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut r = Vec::new();
    r.append(&mut import::routes());
    r.append(&mut config::routes());
    r
}
