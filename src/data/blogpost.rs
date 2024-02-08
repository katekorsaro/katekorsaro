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
    pub fn formatted_date (&self) -> String {
        let date_str = self.date.to_string();
        format!("{}/{}/{}", date_str[..4].to_string(), date_str[4..6].to_string(), date_str[6..8].to_string())
    }
}

pub fn list_of_all_blogposts() -> Vec<BlogPost> {
    vec![
        BlogPost::new("Title", 20240204, "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Vel pretium lectus quam id leo in vitae turpis. Dignissim convallis aenean et tortor at risus viverra adipiscing. Varius quam quisque id diam vel. Fermentum dui faucibus in ornare. Faucibus a pellentesque sit amet porttitor eget dolor. Lorem ipsum dolor sit amet consectetur adipiscing elit duis tristique. Pharetra magna ac placerat vestibulum lectus mauris ultrices eros. Nisl pretium fusce id velit ut. Sed felis eget velit aliquet sagittis id consectetur. Volutpat est velit egestas dui id ornare arcu odio. Massa ultricies mi quis hendrerit. Proin sed libero enim sed faucibus turpis in eu. Eget felis eget nunc lobortis mattis aliquam faucibus. Nibh sit amet commodo nulla. Tempor nec feugiat nisl pretium fusce id velit ut. Nisi vitae suscipit tellus mauris a diam maecenas. Placerat in egestas erat imperdiet sed euismod nisi porta lorem. Sit amet mattis vulputate enim nulla aliquet. Velit ut tortor pretium viverra suspendisse potenti nullam ac. Sagittis orci a scelerisque purus semper. Pharetra magna ac placerat vestibulum lectus mauris ultrices eros in. Nibh cras pulvinar mattis nunc sed blandit libero volutpat. Dignissim sodales ut eu sem integer. Eu feugiat pretium nibh ipsum consequat. Morbi enim nunc faucibus a pellentesque sit amet porttitor. Etiam non quam lacus suspendisse faucibus interdum posuere lorem. Massa massa ultricies mi quis hendrerit dolor magna eget est. Risus quis varius quam quisque id diam. Cursus turpis massa tincidunt dui ut ornare lectus sit. Etiam dignissim diam quis enim lobortis scelerisque. Leo duis ut diam quam nulla porttitor massa. Elit pellentesque habitant morbi tristique senectus et netus et. Amet facilisis magna etiam tempor orci eu lobortis elementum. Feugiat nisl pretium fusce id velit ut. Rhoncus urna neque viverra justo nec ultrices. Amet nisl purus in mollis nunc. Mollis nunc sed id semper risus in hendrerit gravida."),
        BlogPost::new("Title", 20240208, "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Vel pretium lectus quam id leo in vitae turpis. Dignissim convallis aenean et tortor at risus viverra adipiscing. Varius quam quisque id diam vel. Fermentum dui faucibus in ornare. Faucibus a pellentesque sit amet porttitor eget dolor. Lorem ipsum dolor sit amet consectetur adipiscing elit duis tristique. Pharetra magna ac placerat vestibulum lectus mauris ultrices eros. Nisl pretium fusce id velit ut. Sed felis eget velit aliquet sagittis id consectetur. Volutpat est velit egestas dui id ornare arcu odio. Massa ultricies mi quis hendrerit. Proin sed libero enim sed faucibus turpis in eu. Eget felis eget nunc lobortis mattis aliquam faucibus. Nibh sit amet commodo nulla. Tempor nec feugiat nisl pretium fusce id velit ut. Nisi vitae suscipit tellus mauris a diam maecenas. Placerat in egestas erat imperdiet sed euismod nisi porta lorem. Sit amet mattis vulputate enim nulla aliquet. Velit ut tortor pretium viverra suspendisse potenti nullam ac. Sagittis orci a scelerisque purus semper. Pharetra magna ac placerat vestibulum lectus mauris ultrices eros in. Nibh cras pulvinar mattis nunc sed blandit libero volutpat. Dignissim sodales ut eu sem integer. Eu feugiat pretium nibh ipsum consequat. Morbi enim nunc faucibus a pellentesque sit amet porttitor. Etiam non quam lacus suspendisse faucibus interdum posuere lorem. Massa massa ultricies mi quis hendrerit dolor magna eget est. Risus quis varius quam quisque id diam. Cursus turpis massa tincidunt dui ut ornare lectus sit. Etiam dignissim diam quis enim lobortis scelerisque. Leo duis ut diam quam nulla porttitor massa. Elit pellentesque habitant morbi tristique senectus et netus et. Amet facilisis magna etiam tempor orci eu lobortis elementum. Feugiat nisl pretium fusce id velit ut. Rhoncus urna neque viverra justo nec ultrices. Amet nisl purus in mollis nunc. Mollis nunc sed id semper risus in hendrerit gravida.")
    ]
}
