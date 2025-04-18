use crate::*;

#[derive(Debug, Clone)]
/// Represents a structure that stores visited URLs.
///
/// The `Tmp` struct is used to track the URLs that have been visited. It contains a single field,
/// `visit_url`, which is a `HashSet` of strings representing the visited URLs. The use of a `HashSet`
/// ensures that each URL is stored only once, even if it is added multiple times.
///
/// # Fields
/// - `visit_url`: A `HashSet<String>` that stores the URLs that have been visited. This ensures
///   that each URL is unique within the set.
pub struct Tmp {
    pub visit_url: HashSet<String>,
    pub root_cert: RootCertStore,
}
