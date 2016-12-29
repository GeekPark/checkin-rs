use model::*;

#[derive(Debug)]
pub struct Ticket {
    pub id: String,
    pub ticket_cat_id: String,
    pub user_id: String,
    pub qrcode: String,
    pub price: f64,
}

impl Ticket {
    #[rustfmt_skip]
    pub fn create_table(db: &DB) -> () {
        db.create_table("tickets",
                        "id             VARCHAR PRIMARY KEY, \
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

impl Ticket {
    pub fn new(tc_id: &str, uid: &str, qrcode: &str, price: f64) -> Self {
        Ticket {
            id: Uuid::new_v4().hyphenated().to_string(),
            ticket_cat_id: tc_id.into(),
            user_id: uid.into(),
            qrcode: qrcode.into(),
            price: price,
        }
    }
}
