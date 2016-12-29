use rocket::Route;
use rocket::data::Data;
use rocket_contrib::JSON;

use std::collections::HashMap;

use model::*;

#[post("/import/tickets", data = "<data>")]
fn import_tickets(data: Data, db: DBI) -> JSON<HashMap<&'static str, String>> {
    let db = db.0;
    let res = TicketCSVRecord::import(data.open(), &db);
    if let Some(n) = res {
        JSON(map!{
            "ok" => "true".into(),
            "message" => format!("成功導入 {} 條數據", n)
        })
    } else {
        JSON(map!{
            "ok" => "false".into(),
            "message" => "導入大失敗！".into()
        })
    }
}


pub fn routes() -> Vec<Route> {
    routes![import_tickets]
}
