use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text(String::from("This is a post"));
    assert_eq!("", post.contents());

    post.request_review();
    assert_eq!("", post.contents());

    post.approve();
    assert_eq!("This is a post", post.contents());
}
