use std::sync::RwLock;

pub type Day = String;

lazy_static! {
    static ref _GLOBAL_TODAY: RwLock<Day> = RwLock::new(day_for("1"));
}

pub fn day_for(day: &str) -> Day {
    day.into()
}

pub fn get_today() -> Day {
    _GLOBAL_TODAY.read().unwrap().clone()
}

pub fn set_today(today: Day) {
    let mut w = _GLOBAL_TODAY.write().unwrap();
    *w = today;
}

// pub fn contains_today(days: &str) -> bool {
//     let today = get_today();
//     days.split(",").find(|x| x == &today).is_some()
// }
