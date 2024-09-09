#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct User {
    #[serde(rename = "as")]
    pub field_as: String,
    #[serde(rename = "favoriteNumber")]
    #[serde(default = "User::default_favorite_number")]
    pub favorite_number: i32,
    #[serde(default = "User::default_likes_pizza")]
    pub likes_pizza: bool,
    #[serde(default = "User::default_b")]
    pub b: Vec<u8>,
    #[serde(default = "User::default_union_b")]
    pub union_b: Option<Vec<u8>>,
    #[serde(rename = "A_Bool")]
    #[serde(default = "User::default_a_bool")]
    pub a_bool: Vec<bool>,
    #[serde(rename = "SomeInteger")]
    #[serde(default = "User::default_some_integer")]
    pub some_integer: Vec<i32>,
    pub map_of_f64: std::collections::HashMap<String, f64>,
}

impl User {
    #[inline(always)]
    pub fn default_favorite_number() -> i32 { 7 }

    #[inline(always)]
    pub fn default_likes_pizza() -> bool { false }

    #[inline(always)]
    pub fn default_b() -> Vec<u8> { "ÿ".to_string() }

    #[inline(always)]
    pub fn default_union_b() -> Option<Vec<u8>> { None }

    #[inline(always)]
    pub fn default_a_bool() -> Vec<bool> { vec![true, false] }

    #[inline(always)]
    pub fn default_some_integer() -> Vec<i32> { vec![12, -1] }

}

