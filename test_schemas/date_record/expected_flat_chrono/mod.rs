pub mod first {

    #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
    #[serde(default)]
    pub struct DateContainer {
        pub a_date: chrono::NaiveDateTime,
        #[serde(with = "chrono::naive::serde::ts_milliseconds")]
        pub a_time_millis: chrono::NaiveDateTime,
        #[serde(with = "chrono::naive::serde::ts_microseconds")]
        pub a_time_micros: chrono::NaiveDateTime,
        #[serde(with = "chrono::naive::serde::ts_milliseconds")]
        pub a_timestamp_millis: chrono::NaiveDateTime,
        #[serde(with = "chrono::naive::serde::ts_microseconds")]
        pub a_timestamp_micros: chrono::NaiveDateTime,
        pub a_timestamp_nanos: chrono::NaiveDateTime,
        #[serde(with = "chrono::naive::serde::ts_milliseconds")]
        pub a_local_timestamp_millis: chrono::NaiveDateTime,
        #[serde(with = "chrono::naive::serde::ts_microseconds")]
        pub a_local_timestamp_micros: chrono::NaiveDateTime,
        pub a_local_timestamp_nanos: chrono::NaiveDateTime,
    }

    impl DateContainer {}

}
