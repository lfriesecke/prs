pub trait ContentElement {
    fn get_name(&self) -> String;
    fn print(&self);
}

pub struct Heading {
    pub headline: String,
    pub level: u8,
}

impl ContentElement for Heading {
    fn get_name(&self) -> String {
        String::from("Heading")
    }

    fn print(&self) {
        println!("Level :{}: - :{}:", self.level, self.headline);
    }
}

pub struct Text {
    pub text: String,
}

impl ContentElement for Text {
    fn get_name(&self) -> String {
        String::from("Text")
    }

    fn print(&self) {
        println!("Text :{}:", self.text);
    }
}

pub struct UnorderedList {
    pub items: Vec<String>,
}

impl ContentElement for UnorderedList {
    fn get_name(&self) -> String {
        String::from("UnorderedList")
    }

    fn print(&self) {
        for item in self.items.iter() {
            println!("- :{}:", item);
        }
    }
}

pub struct OrderedList {
    pub items: Vec<String>,
}

impl ContentElement for OrderedList {
    fn get_name(&self) -> String {
        String::from("OrderedList")
    }

    fn print(&self) {
        for (pos, item) in self.items.iter().enumerate() {
            println!("{}. :{}:", pos + 1, item);
        }
    }
}