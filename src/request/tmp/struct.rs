use crate::*;

#[derive(Debug, Clone)]
pub struct Tmp {
    pub visit_url: HashSet<String>,
    pub root_cert: RootCertStore,
}
