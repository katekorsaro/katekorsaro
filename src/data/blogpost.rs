#[derive(Clone)]
pub struct BlogPost {
    pub title: String,
    pub date: u32,
    pub text: String,
}

impl BlogPost {
    fn new(title: &str, date: u32, text: &str) -> Self {
        Self {
            title: String::from(title),
            date: date,
            text: String::from(text),
        }
    }
}

pub fn list_of_all_blogposts() -> Vec<BlogPost> {
    vec![BlogPost::new("Title", 20240208, "and this is a test")]
}
