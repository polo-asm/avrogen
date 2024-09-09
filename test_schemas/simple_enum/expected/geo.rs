/// Indicate the direction on the compass.
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Default)]
pub enum CardinalPoints {
    #[default]
    North,
    South,
    East,
    West,
}


