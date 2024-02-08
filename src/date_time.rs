use crate::structs;

pub(crate) fn date_time() -> structs::DateTime {
    let dt: chrono::DateTime<chrono::Local> = chrono::Local::now();
    return structs::DateTime {
        date: Box::new(dt.format("%F")),
        time: Box::new(dt.format("%T")),
    };
}
