use csv::ReaderBuilder;
use std::fs;
use serde_json::Value;

pub fn read_json_as_vec(path: &String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
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

pub fn read_csv_file_as_vec(path: &String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
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