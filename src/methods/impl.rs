use std::fmt::{self, Display};

use super::r#type::Methods;

impl Default for Methods {
    fn default() -> Self {
        Methods::GET
    }
}

impl Methods {
    pub fn new() -> Self {
        Methods::default()
    }

    pub fn is_get(&self) -> bool {
        self.to_owned() == Methods::GET.to_owned()
    }

    pub fn is_post(&self) -> bool {
        self.to_owned() == Methods::POST.to_owned()
    }
}

impl Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Methods::GET => "GET",
            Methods::POST => "POST",
        };
        write!(f, "{}", res)
    }
}
