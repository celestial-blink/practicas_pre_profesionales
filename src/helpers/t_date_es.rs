use time::PrimitiveDateTime;

pub fn month_es(date: &PrimitiveDateTime) -> &'static str {
    match date.month() {
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

pub fn format_date_human_es(date: &PrimitiveDateTime) -> String {
    format!("{} de {} de {}", date.day(), month_es(date), date.year())
}
