use std::result::Result;

use rocket::Route;
use rocket_contrib::JSON;
use serde::ser::{Serialize, Serializer};

use model::*;

#[derive(Serialize)]
struct TicketInfo {
    name: String,
    phone: String,
    company: String,
    position: String,
    email: String,
    ticket_cats: Vec<TicketCat>,
    total_price: f64,
    checked_at: String,
}
struct CheckinResult(Result<TicketInfo, String>);

impl Serialize for CheckinResult {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let &CheckinResult(ref res) = self;
        let mut state = try!(serializer.serialize_map(None));
        try!(serializer.serialize_map_key(&mut state, "ok"));
        try!(serializer.serialize_map_value(&mut state, res.is_ok()));

        match res {
            &Ok(ref tkt) => {
                try!(serializer.serialize_map_key(&mut state, "ticket"));
                try!(serializer.serialize_map_value(&mut state, &tkt));
            }
            &Err(ref e) => {
                try!(serializer.serialize_map_key(&mut state, "error"));
                try!(serializer.serialize_map_value(&mut state, &e));
            }
        }

        serializer.serialize_map_end(state)
    }
}

impl TicketInfo {
    fn from(user: &User, ts: Vec<Ticket>, tcs: Vec<TicketCat>) -> Self {
        use time;
        use std::ops::Add;
        let checked_at = user.checked_at
            .map(time::at)
            .map(|tm| time::strftime("%Y-%m-%d %H:%M:%S", &tm).unwrap())
            .unwrap_or("".into());
        let total_price = ts.iter().map(|x| x.price).fold(0f64, f64::add);
        TicketInfo {
            name: user.name.clone(),
            phone: user.phone.clone(),
            company: user.company.clone(),
            position: user.position.clone(),
            email: user.email.clone(),
            ticket_cats: tcs,
            total_price: total_price,
            checked_at: checked_at,
        }
    }
}

macro_rules! try_err {
    [ $maybe:expr, $err:expr ] => {
        match $maybe {
            Some(sth) => sth,
            None => {
                return JSON(CheckinResult(Err($err.into())))
            }
        }
    }
}


fn result_from_user(mut user: User, db: &DB) -> JSON<CheckinResult> {
    let ts = user.tickets(db);
    let tcs = user.ticket_cats(db);
    try_err!(user.check_in(db), "不可以重复签到");
    try_err!(TicketCat::guard_today(&tcs),
             "用户未购可以进今日会场的票");
    let ticket_info = TicketInfo::from(&user, ts, tcs);
    JSON(CheckinResult(Ok(ticket_info)))
}

#[get("/checkin/code/<code>")]
fn checkin_code(code: &str, db: DBI) -> JSON<CheckinResult> {
    let db = &db.0;
    let ticket = try_err!(Ticket::find_by_qrcode(db, code), "找不到指定票号");
    let user = try_err!(ticket.user(db), "该票无关联用戶");
    result_from_user(user, db)
}

#[get("/checkin/user_id/<user_id>")]
fn checkin_user_id(user_id: &str, db: DBI) -> JSON<CheckinResult> {
    let db = &db.0;
    let user = try_err!(User::find_by_id(db, user_id), "该用户不存在");
    result_from_user(user, db)
}

pub fn routes() -> Vec<Route> {
    routes![checkin_code, checkin_user_id]
}
