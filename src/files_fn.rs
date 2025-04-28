use csv::ReaderBuilder;
use std::fs;
use std::io::{self, Write};
use serde_json::Value;

fn read_json_as_vec(path: &String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let contenido = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&contenido)?;

    // Parseamos el Value a Vec<Vec<String>>
    let table = json.as_array()
        .ok_or("El JSON no es una lista de listas")?
        .iter()
        .map(|row| {
            row.as_array()
                .ok_or("Una fila no es una lista")?
                .iter()
                .map(|v| Ok(v.to_string().trim_matches('"').to_string()))
                .collect::<Result<Vec<String>, Box<dyn std::error::Error>>>()
        })
        .collect::<Result<Vec<Vec<String>>, Box<dyn std::error::Error>>>()?;

    Ok(table)
}
pub fn open_json(table: &mut Vec<Vec<String>>){
    let mut path = String::new();
    print!("Please, insert the JSON file path:");
    io::stdout().flush().unwrap();
    io::stdin().read_line( &mut path).expect("Error! The path wasn't correct");
    path = path.trim().to_string();
    println!("{:?}", path);
    match read_json_as_vec(&path) {
        Ok(value) => {
            println!("SUCCESS! - LOAD JSON FILE");
            table.clear();
            table.extend(value);
        },
        Err(_) => println!("FAILED! - LOAD JSON FILE"),                
    };
}
pub fn test_call_json(path: &String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>>{
    read_json_as_vec(path)
}

fn read_csv_as_vec(path: &String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut file_readed = ReaderBuilder::new()
    .has_headers(false)
    .from_path(path)?;
    
    let mut result = Vec::new();
    for record in file_readed.records() {
        let record = record?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        result.push(row);
    }

    Ok(result)
}
pub fn open_csv(table: &mut Vec<Vec<String>>) {
    let mut path = String::new();
    print!("Please, insert the CSV file path:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut path).expect("Error! The path wasn't correct");
    path = path.trim().to_string();
    println!("{:?}", path);
    match read_csv_as_vec(&path) {
        Ok(value) => {
            println!("SUCCESS! - CSV FILE is loaded");
            table.clear();
            table.extend(value);
        },
        Err(_) => println!("FAILED! - LOAD CSV FILE"),                
    };
}
pub fn test_call_csv(path: &String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>>{
    read_csv_as_vec(path)
}