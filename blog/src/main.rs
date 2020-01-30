extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_test("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("post: {}", post.content());

    post.request_review();
    assert_eq!("", post.content());
    println!("post: {}", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("post: {}", post.content());
}
