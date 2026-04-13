pub struct State {
    pub db: sqlx::MySqlPool,
}

pub mod datetime_format {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::PrimitiveDateTime;
    use time::macros::format_description;

    pub fn serialize<S>(dt: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let s = dt.format(&format).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let formats = [
            format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
            format_description!("[year]-[month]-[day]T[hour]:[minute]"),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
            format_description!("[year]-[month]-[day] [hour]:[minute]"),
        ];

        for format in &formats {
            if let Ok(dt) = PrimitiveDateTime::parse(&s, &format) {
                return Ok(dt);
            }
        }

        Err(serde::de::Error::custom("Invalid date format"))
    }
}

// pub mod modalidad_format {
//     pub fn serialize<S>(modalidad: &i8, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let s = match modalidad {
//             0 => "Preprofesional",
//             1 => "Profesional",
//             2 => "Pre y profesional",
//             _ => "",
//         };
//         serializer.serialize_str(s)
//     }
// }
