#[derive(Clone)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub status: String,
    pub description: String,
    pub status_tag: ProjectStatus,
}

impl Project {
    pub fn new(id: u32, title: &str, status: &str, description: &str, status_tag: ProjectStatus) -> Self {
        Self {
            id,
            title: String::from(title),
            status: String::from(status),
            description: String::from(description),
            status_tag,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ProjectStatus {
    Red,
    Amber,
    Emerald,
    Cyan,
}
