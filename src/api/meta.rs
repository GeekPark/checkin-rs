use rocket::Route;
use rocket_contrib::JSON;

use model::*;

#[derive(Serialize)]
struct Meta {
    total_user_count: Option<usize>,
    checked_user_count: Option<usize>,
}

#[get("/meta")]
fn get_meta(db: DBI) -> JSON<Meta> {
    let db = &db.0;
    JSON(Meta {
        total_user_count: User::count(db),
        checked_user_count: User::checked_count(db),
    })
}

pub fn routes() -> Vec<Route> {
    routes![get_meta]
}
