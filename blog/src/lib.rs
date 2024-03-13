pub struct Post {
    state: Option<Box<dyn State>>,
    contents: String,
}

impl Post {
    pub fn new() -> Self {
        Post{
            state: Some(Box::new(Draft{})),
            contents: String::new(),
        }
    }

    pub fn add_text(&mut self, text: String) {
        self.contents.push_str(&text);
    }

    pub fn contents(&self) -> &str {
        self.state.as_ref().unwrap().contents(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn contents<'a>(&self, _: &'a Post) -> &'a str { "" }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn contents<'a>(&self, post: &'a Post) -> &'a str {
        &post.contents
    }
}
