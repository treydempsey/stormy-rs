use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

pub fn ts_seconds<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>
{
    match <i64>::deserialize(deserializer) {
        Ok(dt) => {
            match NaiveDateTime::from_timestamp_opt(dt, 0) {
                Some(dt) => Ok(dt),
                None => Err(serde::de::Error::custom(format!("value is not a legal timestamp: {}", dt))),
            }
        }
        Err(e) => Err(e),
    }
}

pub mod opt_ts_seconds {
    use chrono::NaiveDateTime;
    use serde::Deserializer;
    use serde::de;
    use std::fmt;

    pub fn deserialize<'de, D>(d: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>
    {
        d.deserialize_option(OptionalNaiveDateTimeVisitor)
    }

    struct OptionalNaiveDateTimeVisitor;

    impl<'de> de::Visitor<'de> for OptionalNaiveDateTimeVisitor {
        type Value = Option<NaiveDateTime>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "null or a datetime string")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            Ok(Some(d.deserialize_u64(NaiveDateTimeVisitor)?))
        }
    }

    struct NaiveDateTimeVisitor;

    impl<'de> de::Visitor<'de> for NaiveDateTimeVisitor {
        type Value = NaiveDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "null2 or a datetime string")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(NaiveDateTime::from_timestamp(value, 0))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(NaiveDateTime::from_timestamp(value as i64, 0))
        }
    }
}
