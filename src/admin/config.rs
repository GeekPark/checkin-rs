use rocket::Route;
use rocket_contrib::JSON;

use std::collections::HashMap;
use model::today::*;

#[get("/set_today/<day>")]
fn set_today_route(day: &str) {
    set_today(day.into());
}

#[get("/today")]
fn get_today_route() -> JSON<HashMap<&'static str, String>> {
    JSON(map!{
        "ok" => "true".into(),
        "today" => get_today()
    })
}


pub fn routes() -> Vec<Route> {
    routes![set_today_route, get_today_route]
}
