pub mod com {

    pub mod example {

        #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
        #[serde(default)]
        pub struct A {
            pub some_int: i32,
            pub some_string: String,
        }

        impl A {}

        #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
        #[serde(default)]
        pub struct B {
            pub some_a: crate::com::example::A,
        }

        impl B {}

    }
}
