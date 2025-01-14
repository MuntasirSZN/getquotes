use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueryResult {
    pub parse: ParseResult,
}

#[derive(Deserialize, Debug)]
pub struct ParseResult {
    pub title: String,
    pub sections: Option<Vec<Section>>,
    #[serde(default)]
    pub text: Option<Text>,
}

#[derive(Deserialize, Debug)]
pub struct Section {
    pub index: String,
    pub number: String,
    pub line: String,
}

#[derive(Deserialize, Debug)]
pub struct Text {
    #[serde(rename = "*")]
    pub content: String,
}
