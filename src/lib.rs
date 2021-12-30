pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.content,
            approvals: 0,
        }
    }
}

pub struct PendingReview {
    content: String,
    approvals: i32,
}

impl PendingReview {
    pub fn approve(mut self) -> Result<Post, PendingReview> {
        self.approvals = self.approvals + 1;
        if self.approvals > 1 {
            return Ok(Post {
                content: self.content,
            });
        }
        Err(self)
    }

    pub fn reprove(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
