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
    pub fn formatted_date(&self) -> String {
        let date_str = self.date.to_string();
        format!(
            "{}/{}/{}",
            date_str[..4].to_string(),
            date_str[4..6].to_string(),
            date_str[6..8].to_string()
        )
    }
}

pub fn list_of_all_blogposts() -> Vec<BlogPost> {
    vec![
        BlogPost::new(
            "A humble beginning...",
            20240208,
            "Hey there, fellow wanderers of the digital realm! Today marks the dawn of a new chapter in my coding \
            saga – diving headfirst into the vibrant sea of open source development. As a humble software dev, \
            I've decided it's high time to roll up my sleeves and contribute to the ever-expanding cosmos of \
            code.\n\
            So, what's got me buzzing with excitement lately? Well, I've found myself knee-deep in the world of \
            Leptos – no, not the diet pills, but rather the captivating open-source project that's been tickling \
            my coding fancy. And guess what? This little site you're browsing right now is the fruit of that \
            laborious tinkering.\n\
            Now, before you start picturing me as some kind of coding superhero, let me set the record straight \
            – I'm just your average dev, armed with nothing but a keyboard, a cup of coffee, and a dash of \
            determination. But hey, who needs a cape when you've got code, right?\n\
            Sure, I may be new to this whole open source gig, but I've got a fire in my belly and a willingness \
            to learn that's second to none. So, if you're a seasoned open source vet stumbling upon this humble \
            abode, consider this an open invitation to lend me your wisdom. And if you're a fellow newbie like \
            myself, well, let's strap ourselves in and embark on this wild ride together.\n\
            They say every journey begins with a single step, and consider this blog post my leap into the \
            unknown. Who knows where this path will lead? But one thing's for sure – I'm ready to roll up my \
            sleeves, get my hands dirty, and make my mark on the open source world, one line of code at a time.\n\
            So here's to new beginnings, endless possibilities, and the thrilling adventure that lies ahead. \
            Let's make some magic happen, shall we?"
        ),
    ]
}
