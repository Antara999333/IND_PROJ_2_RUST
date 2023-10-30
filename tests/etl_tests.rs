use jiwon_shin_sqlite::{extract, query, transform_load};

#[test]
fn test_extract() {
    let url =
        "https://github.com/nogibjj/IDS706_Individual2_PJT/blob/main/data/cereal.csv?raw=true";
    let file_path = "data/cereal.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/cereal.csv";
    let result = transform_load(dataset);

    assert_eq!(result.unwrap(), "cerealDB.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM cerealDB WHERE name = 'Cheerios';";
    let result = query(select_query);

    assert!(result.is_ok());
}
