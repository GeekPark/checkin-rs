use model::*;

#[derive(Debug)]
pub struct Gift {
    pub id: String,
    pub user_id: String,
    pub gift: String,
    pub received_at: Option<Timespec>,
}

impl Gift {
    #[rustfmt_skip]
    pub fn create_table(db: &DB) -> () {
        db.create_table("gifts",
                        "id             VARCHAR PRIMARY KEY, \
                        user_id         VARCHAR NOT NULL, \
                        gift            VARCHAR NOT NULL, \
                        received_at     INTEGER").unwrap();
        db.create_index("gifts", "user_id").unwrap();
        db.create_index("gifts", "gift").unwrap();
    }
}

impl Record for Gift {
    fn static_table_name() -> &'static str {
        "gifts"
    }
    fn static_fields() -> &'static [&'static str] {
        static FIELDS: &'static [&'static str] = &["id", "user_id", "gift", "received_at"];
        FIELDS
    }
    fn values<'a>(&'a self) -> Vec<&'a ToSql> {
        vec![&self.id, &self.user_id, &self.gift, &self.received_at]
    }
    fn from_row(row: &sql::Row) -> Self {
        Gift {
            id: row.get(0),
            user_id: row.get(2),
            gift: row.get(3),
            received_at: row.get(4),
        }
    }
}

impl Gift {
    pub fn new(uid: &str, gift: &str) -> Self {
        Gift {
            id: Uuid::new_v4().hyphenated().to_string(),
            user_id: uid.into(),
            gift: gift.into(),
            received_at: None,
        }
    }

    pub fn already_checked_out(&self) -> bool {
        self.received_at.is_some()
    }
    fn update_column(&mut self, db: &DB, field: &str, val: &ToSql) {
        db.update("gifts",
                  "id = ?",
                  &[&self.id],
                  &format!("{} = ?", field),
                  &[val])
    }
    pub fn check_out(&mut self, db: &DB) -> Option<()> {
        if self.already_checked_out() {
            None
        } else {
            use time::get_time;
            self.update_column(db, "checked_at", &get_time());
            Some(())
        }
    }
    pub fn for_user(db: &DB, uid: &str) -> Vec<String> {
        let gifts = db.search("SELECT * FROM gifts WHERE user_id = ?", &[&uid]);
        let mut names: Vec<String> = gifts.iter().map(|x: &Gift| x.gift.clone()).collect();
        names.sort();
        names.dedup();
        names
    }
    pub fn find_for_user_and_gift(db: &DB, uid: &str, gift: &str) -> Option<Gift> {
        db.search_one("SELECT * FROM gifts WHERE user_id = ? AND gift = ?",
                      &[&uid, &gift])
    }

    pub fn check_out_for(db: &DB, uid: &str, gift: &str) -> Option<()> {
        let record = Self::find_for_user_and_gift(db, uid, gift);
        let mut gift = record.unwrap_or_else(|| {
            let gift = Self::new(uid, gift);
            db.insert(&gift);
            gift
        });
        gift.check_out(db)
    }

    // pub fn user(&self, db: &DB) -> Option<User> {
    //     User::find_by_id(db, &self.user_id)
    // }
}
