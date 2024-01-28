use crate::structs;

pub fn date_time() -> structs::DateTime {
    let dt: chrono::DateTime<chrono::Local> = chrono::Local::now();
    return structs::DateTime {
        date: dt.format("%F").to_string(),
        time: dt.format("%T").to_string(),
    };
}
