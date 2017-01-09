mod checkin;
mod search;
mod gift;
mod meta;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut r = Vec::new();
    r.append(&mut checkin::routes());
    r.append(&mut search::routes());
    r.append(&mut gift::routes());
    r.append(&mut meta::routes());
    r
}
