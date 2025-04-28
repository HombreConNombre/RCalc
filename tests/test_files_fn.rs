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

    // 2. Crear un path temporal
    let mut temp_path = env::temp_dir();
    temp_path.push("test_data.json");

    // 3. Escribir el JSON en el archivo
    let mut file = File::create(&temp_path).expect("Couldn't create temp file");
    file.write_all(test_data.as_bytes()).expect("Couldn't write to temp file");

    // 4. Convertir el PathBuf a String
    let temp_path_str = temp_path.to_str().unwrap().to_string();

    // 5. Llamar a la función
    let result = test_call_json(&temp_path_str);

    // 6. Comprobar que no ha dado error
    assert!(result.is_ok());

    let table = result.unwrap();

    // 7. Comprobar que los datos leídos son correctos
    assert_eq!(table.len(), 3); // 3 filas
    assert_eq!(table[0], vec!["Name", "Age", "Country"]);
    assert_eq!(table[1], vec!["Alice", "30", "USA"]);
    assert_eq!(table[2], vec!["Bob", "25", "UK"]);

    // 8. (Opcional) borrar el archivo temporal si quieres
    std::fs::remove_file(temp_path).expect("Couldn't delete temp file");
}