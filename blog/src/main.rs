use blog::Post;

fn main() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.reject();
  post.approve();
  post.approve();
  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  post.approve();
  assert_eq!("", post.content());
  post.approve();
  post.add_text("I ate a salad for lunch today");
  assert_eq!("I ate a salad for lunch today", post.content());
}
