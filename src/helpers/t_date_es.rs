use time::{OffsetDateTime, PrimitiveDateTime};

pub enum TDate {
    PrimitiveDateTime(PrimitiveDateTime),
    OffsetDateTime(OffsetDateTime),
}

pub fn month_es(date: &TDate) -> &'static str {
    let month = match date {
        TDate::PrimitiveDateTime(date) => date.month(),
        TDate::OffsetDateTime(date) => date.month(),
    };
    match month {
        time::Month::January => "enero",
        time::Month::February => "febrero",
        time::Month::March => "marzo",
        time::Month::April => "abril",
        time::Month::May => "mayo",
        time::Month::June => "junio",
        time::Month::July => "julio",
        time::Month::August => "agosto",
        time::Month::September => "septiembre",
        time::Month::October => "octubre",
        time::Month::November => "noviembre",
        time::Month::December => "diciembre",
    }
}

pub fn format_date_human_es(date: &TDate) -> String {
    match date {
        TDate::PrimitiveDateTime(dat) => {
            format!("{} de {} de {}", dat.day(), month_es(date), dat.year())
        }
        TDate::OffsetDateTime(dat) => {
            format!("{} de {} de {}", dat.day(), month_es(date), dat.year())
        }
    }
}
