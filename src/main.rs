#![feature(plugin)]
#![feature(conservative_impl_trait)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate csv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustc_serialize;
extern crate time;
extern crate uuid;
extern crate docopt;

pub use rocket_contrib::JSON;

mod api;
mod admin;
mod model;
mod cli;

fn init() {
    use model::*;
    let db = DB::connect();
    User::create_table(&db);
    Ticket::create_table(&db);
    TicketCat::create_table(&db);
    TicketCat::seed(&db);
}

fn server() {
    rocket::ignite().mount("/api", api::routes()).launch();
    rocket::ignite().mount("/admin", admin::routes()).launch();
}

fn main() {
    use cli::{parse_argv, CLIAction};
    match parse_argv() {
        Some(CLIAction::Init) => init(),
        Some(CLIAction::Server) => server(),
        None => panic!("Unable to parse cli argument"),
    }
}
