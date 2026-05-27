use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::modules::organizaciones::domain::organizacion::Organizacion;

#[derive(Debug, Clone)]
pub struct CacheState {
    pub organizaciones: Vec<Organizacion>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub db: sqlx::MySqlPool,
    pub cache: CacheState,
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

pub mod datetime_format_option {
    use serde::Serializer;
    use time::PrimitiveDateTime;
    use time::macros::format_description;

    pub fn serialize<S>(dt: &Option<PrimitiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(dt) => {
                let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
                let s = dt.format(&format).map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    // pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     let s = String::deserialize(deserializer)?;
    //     let formats = [
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]"),
    //         format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    //         format_description!("[year]-[month]-[day] [hour]:[minute]"),
    //     ];

    //     for format in &formats {
    //         if let Ok(dt) = PrimitiveDateTime::parse(&s, &format) {
    //             return Ok(Some(dt));
    //         }
    //     }

    //     Err(serde::de::Error::custom("Invalid date format"))
    // }
}

pub mod datetime_no_z {
    use serde::Serializer;
    use time::OffsetDateTime;
    use time::macros::format_description;

    pub fn serialize<S>(dt: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let s = dt.format(&format).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    // pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     let s = String::deserialize(deserializer)?;
    //     let formats = [
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]"),
    //         format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    //         format_description!("[year]-[month]-[day] [hour]:[minute]"),
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z"),
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]Z"),
    //     ];

    //     for format in &formats {
    //         if let Ok(dt) = OffsetDateTime::parse(&s, &format) {
    //             return Ok(dt);
    //         }
    //     }

    //     Err(serde::de::Error::custom("Invalid date format"))
    // }
}

pub mod datetime_no_z_option {
    use serde::Serializer;
    use time::OffsetDateTime;
    use time::macros::format_description;

    pub fn serialize<S>(dt: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(dt) => {
                let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
                let s = dt.format(&format).map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    // pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     let s = String::deserialize(deserializer)?;
    //     let formats = [
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]"),
    //         format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    //         format_description!("[year]-[month]-[day] [hour]:[minute]"),
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z"),
    //         format_description!("[year]-[month]-[day]T[hour]:[minute]Z"),
    //     ];

    //     for format in &formats {
    //         if let Ok(dt) = OffsetDateTime::parse(&s, &format) {
    //             return Ok(Some(dt));
    //         }
    //     }

    //     Err(serde::de::Error::custom("Invalid date format"))
    // }
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

#[derive(Debug, Serialize, FromRow)]
pub struct Total {
    pub total: i32,
}

// todo:
fn deserialize_empty_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    match s {
        None | Some(ref v) if v.is_empty() => Ok(None),
        Some(v) => v.parse::<T>().map(Some).map_err(serde::de::Error::custom),
    }
}
