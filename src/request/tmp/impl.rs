use super::r#type::Tmp;
use rustls::RootCertStore;
use std::collections::HashSet;
use webpki_roots::TLS_SERVER_ROOTS;

impl Default for Tmp {
    fn default() -> Self {
        Self {
            visit_url: HashSet::new(),
            root_cert: RootCertStore {
                roots: TLS_SERVER_ROOTS.to_vec(),
            },
        }
    }
}
