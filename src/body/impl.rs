use crate::*;

impl Default for Body {
    fn default() -> Self {
        Self::Text(EMPTY_STR)
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{}", text.to_string()),
            Self::Json(json) => write!(
                f,
                "{}",
                serde_json::to_string(json).unwrap_or_else(|_| String::from("{}"))
            ),
            Self::Binary(binary) => write!(f, "{:?}", binary),
        }
    }
}

impl Serialize for Body {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Text(text) => text.serialize(serializer),
            Self::Json(json) => json.serialize(serializer),
            Self::Binary(binary) => binary.serialize(serializer),
        }
    }
}
