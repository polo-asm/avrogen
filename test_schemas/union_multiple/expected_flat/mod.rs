pub mod unions {

    #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
    #[serde(default)]
    pub struct Detail {
        pub text: String,
    }

    impl Detail {}

    #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
    #[serde(default)]
    pub struct Message {
        pub content: MessageContent,
        #[serde(default = "Message::default_optional_content")]
        pub optional_content: MessageOptionalContent,
        #[serde(default = "Message::default_with_default")]
        pub with_default: MessageWithDefault,
    }

    impl Message {
        #[inline(always)]
        pub fn default_optional_content() -> MessageOptionalContent {
            MessageOptionalContent::None
        }

        #[inline(always)]
        pub fn default_with_default() -> MessageWithDefault {
            MessageWithDefault::Int(5)
        }
    }

    #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
    #[serde(untagged)]
    pub enum MessageContent {
        Int(i32),
        String(String),
        Detail(crate::unions::Detail),
    }

    impl Default for MessageContent {
        fn default() -> Self {
            MessageContent::Int(Default::default())
        }
    }

    #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
    #[serde(untagged)]
    pub enum MessageOptionalContent {
        None,
        Int(i32),
        String(String),
    }

    impl Default for MessageOptionalContent {
        fn default() -> Self {
            MessageOptionalContent::None
        }
    }

    #[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
    #[serde(untagged)]
    pub enum MessageWithDefault {
        Int(i32),
        String(String),
    }

    impl Default for MessageWithDefault {
        fn default() -> Self {
            MessageWithDefault::Int(Default::default())
        }
    }

}
