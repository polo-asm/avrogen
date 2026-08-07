pub mod first {

    #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
    #[serde(default)]
    pub struct DateContainer {
        #[serde(with = "jiff::fmt::serde::timestamp::second::required")]
        pub a_date: jiff::Timestamp,
        #[serde(with = "jiff::fmt::serde::timestamp::millisecond::required")]
        pub a_time_millis: jiff::Timestamp,
        #[serde(with = "jiff::fmt::serde::timestamp::microsecond::required")]
        pub a_time_micros: jiff::Timestamp,
        #[serde(with = "jiff::fmt::serde::timestamp::millisecond::required")]
        pub a_timestamp_millis: jiff::Timestamp,
        #[serde(with = "jiff::fmt::serde::timestamp::microsecond::required")]
        pub a_timestamp_micros: jiff::Timestamp,
        pub a_timestamp_nanos: jiff::Timestamp,
        #[serde(with = "jiff::fmt::serde::timestamp::millisecond::required")]
        pub a_local_timestamp_millis: jiff::Timestamp,
        #[serde(with = "jiff::fmt::serde::timestamp::microsecond::required")]
        pub a_local_timestamp_micros: jiff::Timestamp,
        pub a_local_timestamp_nanos: jiff::Timestamp,
    }

    impl DateContainer {}

}
