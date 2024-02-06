#[derive(Clone)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub status: String,
    pub description: String,
    pub status_tag: ProjectStatus,
    pub tags: Option<Vec<String>>,
}

impl Project {
    pub fn new(
        id: u32,
        title: &str,
        status: &str,
        description: &str,
        status_tag: ProjectStatus,
    ) -> Self {
        Self {
            id,
            title: String::from(title),
            status: String::from(status),
            description: String::from(description),
            status_tag,
            tags: None,
        }
    }
    pub fn tags(mut self, tags: Vec<&str>) -> Self {
        let tags = tags
            .into_iter()
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        self.tags = Some(tags);
        self
    }
}

#[derive(Clone, PartialEq)]
pub enum ProjectStatus {
    Red,
    Amber,
    Emerald,
    Cyan,
}
