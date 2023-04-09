
pub struct Options {
    current_option: i32,
    list: Vec<Option>
}

struct Option {
    id: i32,
    name: String,
    on_click: fn()
}

impl Options {
    pub fn new() -> Options {
        let options = Options { 
            current_option: 0, 
            list: Vec::new() 
        };

        return options
    }

    pub fn option(&mut self, name: String, on_click: fn()) {
        let id = (self.list.len() + 1) as i32;
        let option = Option {
            id,
            name,
            on_click
        };

        self.list.push(option)
    }
}

