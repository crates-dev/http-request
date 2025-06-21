use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Text(BodyText),
    Json(BodyJson),
    Binary(BodyBinary),
}
