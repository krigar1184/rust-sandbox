pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    pub approves_count: u32,
}


impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approves_count: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            match s.add_text(text) {
                Some(v) => self.content.push_str(v),
                None => self.content.push_str(""),
            };
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().content(&self)
         //match self.state.as_ref() {
         //    Some(v) => v.content(&self),
         //    None => "",
         //}
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn reject(&mut self) {
        self.approves_count = 0;

        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }

    pub fn approve(&mut self) {
        self.approves_count += 1;

        if self.approves_count < 2 {
            return;
        }

        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}


trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn add_text(self: Box<Self>, text: &str) -> Option<&str>;

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}


struct Draft {}


impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(self: Box<Self>, text: &str) -> Option<&str> {
        Some(text)
    }
}


struct PendingReview {}


impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn add_text(self: Box<Self>, _text: &str) -> Option<&str> {
        None
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

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn add_text(self: Box<Self>, _text: &str) -> Option<&str> {
        None
    }
}
