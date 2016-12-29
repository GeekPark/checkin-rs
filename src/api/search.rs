use rocket::Route;
use rocket_contrib::JSON;

use model::*;

#[get("/search/<keyword>")]
fn search(db: DBI, keyword: &str) -> JSON<String> {
    let result: Option<TicketCat> = db.0.search_one("SELECT * FROM ticket_cats;", &[]);
    if let Some(tc) = result {
        JSON(tc.name)
    } else {
        JSON("not found".into())
    }
}


pub fn routes() -> Vec<Route> {
    routes![search]
}
