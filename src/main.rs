use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}


#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}


fn main() {
    let article: Article = Article {
	article: String::from("how to work with json in Rust"),
	author: String::from("tdep"),
	paragraph: vec![
	    Paragraph {
		name: String::from("untyped")
	    },
	    Paragraph {
		name: String::from("strongly typed")
	    },
	    Paragraph {
		name: String::from("writing json")
	    }
	]
    };

    let json = serde_json::to_string(&article).unwrap();

    println!("the JSON is: {}", json)
}