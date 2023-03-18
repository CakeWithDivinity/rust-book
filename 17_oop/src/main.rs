use oop::{Screen, SelectBox, Button, Post};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    "yes".to_string(),
                    "maybe".to_string(),
                    "no".to_string(),
                ]            
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "Ok".to_string()
            })
        ],
    };

    screen.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();

    post.add_text(" do not append");

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
