use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}


fn read_json_typed(json: &str) -> Article {
    let parsed = serde_json::from_str(json).unwrap();

    return parsed;
}

fn main() {
    let json = r#"
        {
            "article": "Test",
            "author": "Gugu",
            "paragraphs": [
                {
                    "name": "pwpppwp"
                }
            ]
        }
    "#;

    let parsed: Article = read_json_typed(json);

    println!("\n\n primeiro paragrafo: {}", parsed.paragraphs[0].name);
}
