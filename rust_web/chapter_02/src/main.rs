// fn main() {
//     struct Book {
//         title: String,
//         isbn: Option<String>,
//     }

//     let book = Book {
//         title: "Great book".to_string(),
//         isbn: Some(String::from("1-123-456")),
//     };

//     match book.isbn {
//         Some(i) => println!("The ISBN of the book: {} is: {}", book.title, i),
//         None => println!("We don't know the ISBN of the book"),
//     }
// }
use warp::Filter;

use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug)]
struct Question {
    id : QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);

impl Question {
    fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>
    ) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter)
        -> Result<(), std::fmt::Error> {
            write!(
                f,
                "{}, title: {}, content: {}, tags: {:?}",
                self.id, self.title, self.content, self.tags
            )
        }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter)
        -> Result<(), std::fmt::Error> {
            write!(f, "id: {}", self.0)
        }
    }

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "No id provided")
            ),
        }
    }
}

// fn main() {
//     let question = Question::new(
//         QuestionId::from_str("1").expect("No id provided"),
//         "First Question".to_string(),
//         "Content of question".to_string(),
//         Some(vec!["faq".to_string()]),
//     );
//     println!("{:?}", question);
// }

#[tokio::main]
async fn main() {
    let hello = warp::get().map(||format!("Hello, World!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn hello() -> &'static str {
    "Hello, World!"
}