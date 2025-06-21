use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Body {
    Text(BodyText),
    Json(BodyJson),
    Binary(BodyBinary),
}
