use std::io::Read;

use csv;

use model::*;

pub struct Ticket {
    id: String,
    ticket_cat_id: String,
    user_id: String,
    qrcode: String,
    price: f64,
}

#[derive(RustcDecodable)]
struct CSVRecord {
    name: String,
    phone: String,
    company: String,
    position: String,
    email: String,
    qrcode: String,
    ticket_cat: String,
    price: Option<f32>,
    ticket_cat_id: Option<String>,
    note: Option<String>,
    status: String,
}

impl Ticket {
    pub fn import_from_csv<T>(csv: T) -> Option<usize>
        where T: Read
    {
        let mut rdr = csv::Reader::from_reader(csv);
        let mut count = 0;
        for record in rdr.decode() {
            let r: CSVRecord = record.unwrap();

            count += 1;
        }
        Some(count)
    }

    #[rustfmt_skip]
    pub fn create_table(db: &DB) -> () {
        db.create_table("tickets",
                        "id             INTEGER PRIMARY KEY, \
                        ticket_cat_id   VARCHAR NOT NULL, \
                        user_id         VARCHAR NOT NULL, \
                        qrcode          VARCHAR NOT NULL, \
                        price           REAL");
        db.create_index("tickets", "ticket_cat_id");
        db.create_index("tickets", "user_id");
        db.create_index("tickets", "qrcode");
    }
}

impl Record for Ticket {
    fn static_table_name() -> &'static str {
        "tickets"
    }
    fn static_fields() -> &'static [&'static str] {
        static FIELDS: &'static [&'static str] =
            &["id", "ticket_cat_id", "user_id", "qrcode", "price"];
        FIELDS
    }
    fn values<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![&self.id, &self.ticket_cat_id, &self.user_id, &self.qrcode, &self.price]
    }
    fn from_row(row: &sql::Row) -> Self {
        Ticket {
            id: row.get(0),
            ticket_cat_id: row.get(1),
            user_id: row.get(2),
            qrcode: row.get(3),
            price: row.get(4),
        }
    }
}
