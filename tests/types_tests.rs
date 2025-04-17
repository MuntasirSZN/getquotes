use getquotes::types::QueryResult;
use serde_json::json;

#[test]
fn test_query_result_deserialization() {
    let json_data = json!({
        "parse": {
            "title": "Test Title",
            "sections": [
                {
                    "index": "1",
                    "number": "1",
                    "line": "First Section"
                },
                {
                    "index": "2",
                    "number": "2",
                    "line": "Second Section"
                }
            ],
            "text": {
                "*": "Some HTML content"
            }
        }
    });

    let query_result: QueryResult = serde_json::from_value(json_data).unwrap();

    assert_eq!(query_result.parse.title, "Test Title");
    assert!(query_result.parse.sections.is_some());

    let sections = query_result.parse.sections.unwrap();
    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].index, "1");
    assert_eq!(sections[0].number, "1");
    assert_eq!(sections[0].line, "First Section");

    assert_eq!(sections[1].index, "2");
    assert_eq!(sections[1].number, "2");
    assert_eq!(sections[1].line, "Second Section");

    assert!(query_result.parse.text.is_some());
    assert_eq!(
        query_result.parse.text.unwrap().content,
        "Some HTML content"
    );
}

#[test]
fn test_query_result_without_sections() {
    let json_data = json!({
        "parse": {
            "title": "Test Title",
            "text": {
                "*": "Some HTML content"
            }
        }
    });

    let query_result: QueryResult = serde_json::from_value(json_data).unwrap();

    assert_eq!(query_result.parse.title, "Test Title");
    assert!(query_result.parse.sections.is_none());

    assert!(query_result.parse.text.is_some());
    assert_eq!(
        query_result.parse.text.unwrap().content,
        "Some HTML content"
    );
}

#[test]
fn test_query_result_without_text() {
    let json_data = json!({
        "parse": {
            "title": "Test Title",
            "sections": [
                {
                    "index": "1",
                    "number": "1",
                    "line": "First Section"
                }
            ]
        }
    });

    let query_result: QueryResult = serde_json::from_value(json_data).unwrap();

    assert_eq!(query_result.parse.title, "Test Title");
    assert!(query_result.parse.sections.is_some());

    let sections = query_result.parse.sections.unwrap();
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].line, "First Section");

    assert!(query_result.parse.text.is_none());
}
