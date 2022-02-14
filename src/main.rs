#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use ftp::FtpStream as FtpServer;
use std::fs::File;
use std::io;
use std::io::Read;
struct UpdateData {
    supported_models: Vec<String>,
    date_code: String,
    supported_langs: String,
}
fn main() {
    match send_update() {
        Ok(_) => println!("Success. Your printer should say \"PROGRAMING\""),
        Err(e) => eprintln!("{e}"),
    };
}

fn send_update() -> Result<(), String> {
    let stdin = io::stdin();
    println!("Please enter the location of the update file");
    let mut file_location = String::new();
    stdin
        .read_line(&mut file_location)
        .map_err(|e| format!("Could not read from stdin. [{e}]"))?;
    file_location = file_location.trim().to_string();
    let mut f = File::open(&file_location)
        .map_err(|e| format!("Could not open file: {} [{e}]", &file_location))?;
    // read up to 10 kilabytes (the first PJL section plus some incase rfu files are diferent)
    let mut buffer = [0; 10000];

    f.read(&mut buffer)
        .map_err(|e| format!("Could not read from file {} [{e}]", &file_location))?;
    let update_data = parse_rfu_bytes(&buffer)?;

    println!("Loaded file: {}\nSupported models:", &file_location);
    for i in update_data.supported_models {
        println!("{i}");
    }
    println!(
        "datecode: {}\nSupported langs: {}",
        update_data.date_code, update_data.supported_langs
    );
    println!("Please enter your printers ip: ");
    let mut buffer = String::new();
    stdin
        .read_line(&mut buffer)
        .map_err(|e| format!("Could not read from stdin. [{e}]"))?;
    let mut conn = FtpServer::connect(buffer.trim().to_string() + ":21")
        .map_err(|e| format!("Could not connect to printer. Is tftp enabled? [{e}]"))?;
    conn.put("update.rfu", &mut f)
        .map_err(|e| format!("Could not send file to printer. Try again. [{e}]"))?;
    conn.quit()
        .map_err(|e| format!("Could not close connection succesfully. [{e}]"))?;
    Ok(())
}

///Returns some information about the update file
fn parse_rfu_bytes(file: &[u8]) -> Result<UpdateData, String> {
    let string = String::from_utf8_lossy(file);
    let split_char = "\u{001B}%-12345X"; //<ESC>%-12345X Known as the PJL Universal Exit Charecter
    
    //This is probabaly inefficent and looks bad. We should find a way to make it look better or at least more efficent.
    let info_parts = string
        .split_once(split_char)
        .unwrap()
        .1 //Enter PJL
        .split_once(split_char)
        .unwrap()
        .0; //Exit PJL
    let vec_data: Vec<&str> = info_parts
        .lines()
        .filter(|x| x.starts_with("@PJL COMMENT "))
        .map(|x| x.strip_prefix("@PJL COMMENT ").unwrap())
        .collect();
    let mut date_code = String::new();
    let mut supported_langs = String::new();
    let mut supported_models = vec![];
    for i in vec_data {
        let (first, second) = i.split_once('=').unwrap();
        match first {
            "LOCVERSION" |
            "VERSION" => (),
            "DATECODE" => date_code = second.to_string(),
            "MODEL" => supported_models.push(second.to_string()),
            "LANGUAGEGROUP" => supported_langs = second.to_string(),
            _ => return Err(format!("Unexpected PJL comment value {first}")),
        };
    }
    Ok(UpdateData {
        supported_models,
        date_code,
        supported_langs,
    })
}
