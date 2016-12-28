mod import;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut r = Vec::new();
    r.append(&mut import::routes());
    r
}
