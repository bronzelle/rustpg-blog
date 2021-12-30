use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let mut post = post.request_review();

    let post_approved: Post;

    loop {
        match post.approve() {
            Ok(p) => {
                post_approved = p;
                break;
            }
            Err(p) => post = p,
        }
    }

    assert_eq!("I ate a salad for lunch today", post_approved.content());
}
