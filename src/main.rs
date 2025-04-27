use std::io::{self, Write};
mod files_fn;
// We are going to crate the array base


fn draw_table(table: &Vec<Vec<String>>){
    clear_screen();
    for row in table {
        for celd in row {
            print!("{:<12}", celd); // Align center
        }
        println!();
    }
}

fn load_data_table( table: &mut Vec<Vec<String>>){
    table.push(vec![
        "Nombre".to_string(),
        "Edad".to_string(),
        "Ciudad".to_string(),
    ]);
    table.push(vec![
        "Ana".to_string(),
        "30".to_string(),
        "Madrid".to_string(),
    ]);
    table.push(vec![
        "Luis".to_string(),
        "25".to_string(),
        "".to_string(), // Valor vacío para probar
    ]);
    table.push(vec![
        "Clara".to_string(),
        "".to_string(), // Edad vacía
        "Valencia".to_string(),
    ]);
}

fn load_data() -> String {
    let mut data = String::new();

    print!("Insert the data:");
    io::stdout().flush().expect("Error in flush!");
    io::stdin()
        .read_line(&mut data)
        .expect("The data wasn't loaded right");
    
    println!("Data {}!", data.trim());

    return data.trim().to_string()
}

fn select_celd() -> Vec<i16>{
    let mut row = String::new();
    let mut column = String::new();
    let mut value_list = vec![];

    print!("Insert the row:");
    io::stdout().flush().expect("Error in flush!");
    io::stdin()
        .read_line(&mut row)
        .expect("The data type wasn't right.");
    
    print!("Insert the column:");
    io::stdout().flush().expect("Error in flush!");
    io::stdin()
        .read_line(&mut column)
        .expect("The data type wasn't right.");
    
    match row.trim().parse::<i16>(){
        Ok(row_match) => value_list.push(row_match),
        Err(_) => println!("Error: The data couldn't be transform.")
    }

    match column.trim().parse::<i16>(){
        Ok(column_match) => value_list.push(column_match),
        Err(_) => println!("Error: The data couldn't be transform.")
    }
    return value_list;
}

fn set_data(table: &mut Vec<Vec<String>>){
    if table.len() > 0 {
        loop {
            let position = select_celd();
            if table.len() > (position[0] as usize){
                if table.len() > (position[1] as usize){
                    let data = load_data();
                    table[position[0] as usize][position[1] as usize] = data; 
                    break;
                }
            }    
        }
    }else{
        let data = load_data();
        let vec_data: Vec<String> = vec![data];
        table.push(vec_data);
    }
}

fn main() {
    let mut table: Vec<Vec<String>> = Vec::new();

    loop {
        // Menu loop
        println!("\n----- Menu -----");
        println!("1. Files");
        println!("2. Data");
        println!("3. Exit");
        
        print!("Choose an option, please: ");
        io::stdout().flush().unwrap();
        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Error: the option isn't valid");
        let int_option: i8 = option.trim().parse().unwrap_or(3);

        match &int_option {
            1 => {
                println!("File menu");
                io::stdout().flush().unwrap();
                files_menu(&mut table);
                draw_table( &table);
            },
            2 => {
                println!("Load data menu");
                set_data(&mut table);
            },
            3 => {
                println!("Thank you for use me");
                break; // the software is finished
            },
            _ => println!("Invalid option!"),
        }
        clear_screen();
        draw_table( &table);
    }    
}

fn files_menu(table: &mut Vec<Vec<String>>){
    clear_screen();
    let mut option = String::new();
    let mut path = String::new();

    println!("\n----- Menu -----");
    println!("1. Load JSON");
    println!("2. Load CSV");
    println!("3. Exit");
    
    print!("Choose an option, please: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut option).expect("Error: the option isn't valid");
    let option: i8 = option.trim().parse().unwrap_or(0);

    match option {
        1 => {
            println!("Loading JSON");
            print!("Please, insert the JSON file path:");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut path).expect("Error! The path wasn't correct");
            path = path.trim().to_string();
            println!("{:?}", path);
            match files_fn::read_json_as_vec(&path) {
                Ok(value) => {
                    println!("SUCCESS! - LOAD JSON FILE");
                    table.clear();
                    table.extend(value);
                },
                Err(_) => println!("FAILED! - LOAD JSON FILE"),                
            };
        },
        2 => {
            println!("Loading CSV");
            print!("Please, insert the CSV file path:");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut path).expect("Error! The path wasn't correct");
            path = path.trim().to_string();
            println!("{:?}", path);
            match files_fn::read_csv_file_as_vec(&path) {
                Ok(value) => {
                    println!("SUCCESS! - CSV FILE is loaded");
                    table.clear();
                    table.extend(value);
                },
                Err(_) => println!("FAILED! - LOAD CSV FILE"),                
            };
        }
        3 => println!("Leaving from files menu."),
        _ => println!("Invalid option!"),
    }
    io::stdout().flush().unwrap();
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        // 'cls' is used on Windows
        std::process::Command::new("cls").status().unwrap();
    } else {
        // 'clear' is used on Unix based OS 
        std::process::Command::new("clear").status().unwrap();
    }
}