pub struct Project {
    pub title: String,
    pub status: String,
    pub description: String,
    pub status_tag: ProjectStatus,
    pub tags: Option<Vec<String>>,
    pub project_url: Option<String>,
}

impl Project {
    pub fn new(title: &str, status: &str, description: &str, status_tag: ProjectStatus) -> Self {
        Self {
            title: String::from(title),
            status: String::from(status),
            description: String::from(description),
            status_tag,
            tags: None,
            project_url: None,
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
    pub fn project_url(mut self, project_url: &str) -> Self {
        self.project_url = Some(String::from(project_url));
        self
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum ProjectStatus {
    Red,
    Amber,
    Emerald,
    Cyan,
}
