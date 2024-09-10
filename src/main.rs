use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::Filter;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "")),
        }
    }
}

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
            tags
        }
    }
}

// Creates our first route handler, which needs to return a reply and rejection for Warp to be able to use it
// warp::Reply for the success part
// warp::Rejection for the error
async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    // Creates a new question, which we return to the requesting client
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()])
    );
    // Uses Warp’s json reply to return the JSON version of our question
    Ok(warp::reply::json(&question))
}


#[tokio::main]
async fn main() {
    // Uses Warp’s functionality of chaining more than one filter via .and, and therefore creates one big filter and assigns it to get_items
    // Uses path::end to signal that we listen on exactly /question (and not /question/further/params, for example)

    //  We start with the get filter, which filters all HTTP requests with the GET method. Then we add a path, which filters HTTP requests for the parameters after the host URL
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);
    // Defines the routes variable, which will come in handy later
    let routes = get_items;

    // Passes the routes filter to Warp’s serve method and starts our server
    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}
