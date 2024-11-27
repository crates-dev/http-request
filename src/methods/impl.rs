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

    pub fn value(&self) -> &str {
        match self {
            Methods::GET => "GET",
            Methods::POST => "POST",
        }
    }

    pub fn is_get(&self) -> bool {
        self.value() == Methods::GET.value()
    }

    pub fn is_post(&self) -> bool {
        self.value() == Methods::POST.value()
    }
}
