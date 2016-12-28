mod checkin;
mod search;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut r = Vec::new();
    r.append(&mut checkin::routes());
    r.append(&mut search::routes());
    r
}
