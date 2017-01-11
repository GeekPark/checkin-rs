use std::result::Result;

use rocket::Route;
use rocket_contrib::JSON;
use serde::ser::{Serialize, Serializer};

use model::*;

#[derive(Serialize)]
struct UserInfo {
    id: String,
    name: String,
    phone: String,
    company: String,
    position: String,
    email: String,
    tickets: Vec<TicketInfo>,
    checked_at: String,
}

#[derive(Serialize)]
struct TicketInfo {
    name: String,
    days: String,
    price: f64,
    free: bool,
}

struct CheckinResult<T>(Result<T, String>);

impl<T> Serialize for CheckinResult<T>
    where T: Serialize
{
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

impl UserInfo {
    fn from(user: &User, ts: Vec<Ticket>, tcs: Vec<TicketCat>) -> Self {
        use time;
        let checked_at = user.checked_at
            .map(time::at)
            .map(|tm| time::strftime("%Y-%m-%d %H:%M:%S", &tm).unwrap())
            .unwrap_or("".into());
        let tis: Vec<TicketInfo> = ts.into_iter().zip(tcs).map(TicketInfo::from).collect();
        UserInfo {
            id: user.id.clone(),
            name: user.name.clone(),
            phone: user.phone.clone(),
            company: user.company.clone(),
            position: user.position.clone(),
            email: user.email.clone(),
            tickets: tis,
            checked_at: checked_at,
        }
    }
}

impl TicketInfo {
    fn from(ttc: (Ticket, TicketCat)) -> Self {
        let (ref t, ref tc) = ttc;
        TicketInfo {
            name: tc.name.clone(),
            days: tc.days.clone(),
            price: t.price,
            free: Self::is_free((t, tc)),
        }
    }

    fn is_free((t, tc): (&Ticket, &TicketCat)) -> bool {
        if tc.name == "极客超级票" || tc.name == "VIP 票" {
            false
        } else {
            t.price < 11f64
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


fn result_from_user(mut user: User, db: &DB) -> JSON<CheckinResult<UserInfo>> {
    let ts = user.tickets(db);
    let tcs = user.ticket_cats(db);
    try_err!(user.check_in(db), "不可以重复签到");
    // try_err!(TicketCat::guard_today(&tcs),
    //         "用户未购可以进今日会场的票");
    let user_info = UserInfo::from(&user, ts, tcs);
    JSON(CheckinResult(Ok(user_info)))
}

#[get("/checkin/code/<code>")]
fn checkin_code(code: &str, db: DBI) -> JSON<CheckinResult<UserInfo>> {
    let db = &db.0;
    let ticket = try_err!(Ticket::find_by_qrcode(db, code), "找不到指定票号");
    let user = try_err!(ticket.user(db), "该票无关联用戶");
    result_from_user(user, db)
}

#[get("/checkin/user_id/<user_id>")]
fn checkin_user_id(user_id: &str, db: DBI) -> JSON<CheckinResult<UserInfo>> {
    let db = &db.0;
    let user = try_err!(User::find_by_id(db, user_id), "该用户不存在");
    result_from_user(user, db)
}

#[get("/uncheck/code/<code>")]
fn uncheck_code(code: &str, db: DBI) -> JSON<CheckinResult<i32>> {
    let db = &db.0;
    let ticket = try_err!(Ticket::find_by_qrcode(db, code), "找不到指定票号");
    let user = try_err!(ticket.user(db), "该票无关联用戶");
    JSON(CheckinResult(Ok(try_err!(user.uncheck(db), "用户尚未签到"))))
}

#[get("/uncheck/user_id/<user_id>")]
fn uncheck_user_id(user_id: &str, db: DBI) -> JSON<CheckinResult<i32>> {
    let db = &db.0;
    let user = try_err!(User::find_by_id(db, user_id), "该用户不存在");
    JSON(CheckinResult(Ok(try_err!(user.uncheck(db), "用户尚未签到"))))
}

pub fn routes() -> Vec<Route> {
    routes![checkin_code, checkin_user_id, uncheck_code, uncheck_user_id]
}
