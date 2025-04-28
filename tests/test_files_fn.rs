use std::fs::File;
use std::io::Write;
use std::env;
use rcalc::files_fn::*;

#[test]
fn test_read_json_as_vec(){
    let test_data = r#"
        [
            ["Name", "Age", "Country"],
            ["Alice", "30", "USA"],
            ["Bob", "25", "UK"]
        ]
    "#;

    let mut temp_path = env::temp_dir();
    temp_path.push("test_data.json");

    let mut file = File::create(&temp_path).expect("Couldn't create temp file");
    file.write_all(test_data.as_bytes()).expect("Couldn't write to temp file");

    let temp_path_str = temp_path.to_str().unwrap().to_string();

    let result = test_call_json(&temp_path_str);

    assert!(result.is_ok());

    let table = result.unwrap();

    assert_eq!(table.len(), 3);
    assert_eq!(table[0], vec!["Name", "Age", "Country"]);
    assert_eq!(table[1], vec!["Alice", "30", "USA"]);
    assert_eq!(table[2], vec!["Bob", "25", "UK"]);

    std::fs::remove_file(temp_path).expect("Couldn't delete temp file");
}

#[test]
fn test_read_csv_as_vec(){
    let test_data = "Name,Age,Location\nAlice,28,New York\nBob,22,Los Angeles\nCharlie,30,San Francisco\n";

    let mut temp_path = env::temp_dir();
    temp_path.push("test_data.csv");

    let mut file = File::create(&temp_path).expect("Couldn't create temp file");
    file.write_all(test_data.as_bytes()).expect("Couldn't write to temp file");

    let temp_path_str = temp_path.to_str().unwrap().to_string();

    let result = test_call_csv(&temp_path_str);

    assert!(result.is_ok());

    let table = result.unwrap();

    assert_eq!(table.len(), 4);
    assert_eq!(table[0], vec!["Name", "Age", "Location"]);
    assert_eq!(table[1], vec!["Alice", "28", "New York"]);
    assert_eq!(table[2], vec!["Bob", "22", "Los Angeles"]);
    assert_eq!(table[3], vec!["Charlie","30","San Francisco"]);
    
    std::fs::remove_file(temp_path).expect("Couldn't delete temp file");
}