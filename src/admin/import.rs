use rocket::Route;
use rocket::data::Data;

use model::*;

#[post("/import/tickets", data = "<data>")]
fn import_tickets(data: Data) -> &'static str {
    if Ticket::import_from_csv(data.open()).is_some() {
        "success"
    } else {
        "failed"
    }
}


pub fn routes() -> Vec<Route> {
    routes![import_tickets]
}
