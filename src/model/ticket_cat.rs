use model::*;

#[derive(Serialize)]
pub struct TicketCat {
    pub id: String,
    pub name: String,
    pub days: String,
}

impl Record for TicketCat {
    fn static_table_name() -> &'static str {
        "ticket_cats"
    }
    fn static_fields() -> &'static [&'static str] {
        static FIELDS: &'static [&'static str] = &["id", "name", "days"];
        FIELDS
    }
    fn values<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![&self.id, &self.name, &self.days]
    }
    fn from_row(row: &sql::Row) -> Self {
        TicketCat {
            id: row.get(0),
            name: row.get(1),
            days: row.get(2),
        }
    }
}

impl TicketCat {
    fn new(id: &str, name: &str, days: &str) -> Self {
        TicketCat {
            id: Uuid::parse_str(id).unwrap().hyphenated().to_string(),
            name: name.into(),
            days: days.into(),
        }
    }
    #[rustfmt_skip]
    pub fn create_table(db: &DB) {
        db.create_table("ticket_cats",
                        "id VARCHAR PRIMARY KEY, \
                         name VARCHAR NOT NULL, \
                         days VARCHAR NOT NULL")
          .unwrap()
    }

    #[rustfmt_skip]
    pub fn seed(db: &DB) {
        let v =
        vec![Self::new("2135ee59c4144ff28d46092a345b34c2", "极客狂欢票", "3"),
             Self::new("aa5ff07b050d4ba4b129f80b02f9e85a", "极客探索票", "2"),
             Self::new("3c60344f4fc046f5b0efc22d682e30a0", "极客探索票", "2"),
             Self::new("911ab3f9fe394609b7c7d01e2751c7ea", "极客狂欢票", "3"),
             Self::new("0c79bbf831bc47bdb9142207197e11d5", "极客超级票", "1,2,3"),
             Self::new("88321ed060cd439eb8fc9aec9f7cf624", "极客趣享票", "1"),
             Self::new("c87b857ad1434da2b241fb9d35f05cc9", "极客先锋票", "1,2,3"),
             Self::new("7797d64ca97748ec8746e4ae980734ab", "极客趣享票", "1"),
             Self::new("08e671f9f40b4020bcf94f4e05eda2b5", "极客体验票", "1"),
             // Self::new("11111111111111111111111111111111", "媒体票",    "1,2,3"),
             Self::new("b303ec524bca43ffb3582a4d1d2bd660", "媒体票",    "1,2,3")];
        for rec in v.iter() {
            db.insert(rec);
        }
    }

    pub fn media_ticket(db: &DB) -> Self {
        Self::find_by_name(db, "媒体票").unwrap()
    }

    pub fn find_by_name(db: &DB, name: &str) -> Option<Self> {
        db.search_one("SELECT * FROM ticket_cats WHERE name = ?", &[&name])
    }

    pub fn find_by_id(db: &DB, id: &str) -> Option<Self> {
        let id = Uuid::parse_str(id).unwrap().hyphenated().to_string();
        db.search_one("SELECT * FROM ticket_cats WHERE id = ?", &[&id])
    }

    // pub fn guard_today(vec: &Vec<Self>) -> Option<()> {
    //     if vec.iter().any(|tc| today::contains_today(&tc.days)) {
    //         Some(())
    //     } else {
    //         None
    //     }
    // }
}
