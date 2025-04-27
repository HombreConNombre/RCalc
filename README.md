# 📋 RCalc

RCalc is a lightweight spreadsheet-style table manager built in Rust.
It allows you to load, view, edit, and save data from JSON and CSV files directly through a simple terminal interface.
# 🚀 Features

    Load .json or .csv files into memory.

    Display the current table (as Vec<Vec<String>>).

    Insert, edit, and delete table entries.

    Save the current table back into a .json file.

    Console-based interactive menu with input validation.

    Basic language support system planned (for translations).

# ⚙️ Installation

## Clone the repository:
    git clone git@github.com:your-user/rcalc.git
    cd rcalc
## Make sure you have Rust installed:
    rustup --version
    cargo --version
## Build and run:
    cargo build && cargo run
# 🛠️ Technologies Used

    Rust (Vanilla, without external frameworks)

    serde_json for JSON handling

    csv crate for CSV file management

    Standard Rust libraries (std::fs, std::io, etc.)

# 🧪 Testing

## Basic tests are included for:

    Reading and validating JSON files

    Reading and validating CSV files

## Planned:

    Tests for saving tables to JSON

    Tests for insertion, editing, and deletion operations

# ✨ Future Improvements

    Full multi-language support using dictionaries (HashMaps).

    Export table data as .csv.

    Smarter user input validation.

    Add configuration file support.

    More automated tests and CI integration.
    
