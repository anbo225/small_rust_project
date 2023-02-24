//! State is a behavioral design pattern that lets an object alter its behavior when its internal state changes.
//! It appears as if the object changed its class.

//! We’ll implement a blog post workflow
//! 1. A blog post starts as an empty draft.
//! 2. When the draft is done, a review of the post is requested.
//! 3. When the post is approved, it gets published.
//! 4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
//! 5 Add a reject method that changes the post’s state from PendingReview back to Draft.
//! 6 Require two calls to approve before the state can be changed to Published.
//! 7 Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.

/// Examples: using traint object to implement state pattern
/// State: Draft , PendingReview, Published;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    number_of_approvals: i32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            number_of_approvals: 0,
        }
    }

    pub fn content(&self) -> &str {
        // todo: some bug happens here
        &self.state.as_ref().unwrap().content(self)
    }

    pub fn add_text(&mut self, text: &str) {
        self.content = self.state.as_ref().unwrap().add_text(&self.content, text)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(self))
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn add_text(&self, original_text: &str, _text_to_add: &str) -> String {
        original_text.to_string()
    }

    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, original_text: &str, _text_to_add: &str) -> String {
        format!("{}{}", original_text, _text_to_add)
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        if post.number_of_approvals < 1 {
            post.number_of_approvals = post.number_of_approvals + 1;
            self
        } else {
            Box::new(Published {})
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_add() {
        let mut post = Post::new();
        assert_eq!(post.content(), "");

        post.add_text("Hello");
        post.add_text(" World");
        assert_eq!(post.content(), "");

        post.approve();

        post.request_review();
        post.add_text(" World");
        assert_eq!(post.content(), "");
        post.add_text(" World");

        post.request_review();
        post.approve();
        post.add_text(" World");
        post.add_text(" World");
        assert_eq!(post.content(), "");

        post.request_review();
        post.approve();
        assert_eq!(post.content(), "Hello World");
    }
}
