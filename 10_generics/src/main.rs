use std::fmt::{Display, Debug};

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest(&number_list));

    let string1 = "abcd".to_string();
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

}

pub trait Summary {
    fn summarize(&self) -> String {
        "(Read more...)".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T,U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{ 42 }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {
            x,
            y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}" , self.y);
        }
    }
}



