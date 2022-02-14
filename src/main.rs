use ftp::FtpStream as Ftpcmd;
use std::io;
#[derive(Debug)]
struct UpdateData {
    supported_models: Vec<String>,
    date_code: String,
    supported_langs: String,
}
fn main() {
    println!("This utility will revert your HP printer to a version that supports cartriges.");
    println!("Please enter your printers ip: ");
    match send_update() {
        Ok(_) => println!("Success. Your printer should say \"PROGRAMING\""),
        Err(e) => eprintln!("{e}"),
    };
}

fn send_update() -> Result<(), String> {
    let file = include_bytes!("update.rfu");
    println!("{:?}", parse_file(file)?);
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .map_err(|e| format!("Could not read from stdin. [{e}]"))?;
    let mut conn = Ftpcmd::connect(buffer.trim().to_owned() + ":21")
        .map_err(|e| format!("Could not connect to printer. Is tftp enabled? [{e}]"))?;
    conn.put("update.rfu", &mut std::io::Cursor::new(file))
        .map_err(|e| format!("Could not send file to printer. Try again. [{e}]"))?;
    conn.quit()
        .map_err(|e| format!("Could not close connection succesfully. [{e}]"))?;
    Ok(())
}

fn parse_file(file: &[u8]) -> Result<UpdateData, String> {
    let string = String::from_utf8_lossy(file);
    let split_char = "%-12345X";//Known as the PJL Universal Exit Charecter
    
    //This is probabaly inefficent and looks bad. We should find a way to make it look better or at least more efficent.
    let info_parts = string
        .split_once(split_char)
        .unwrap()
        .1
        .split_once(split_char)
        .unwrap()
        .0;
    let mut vec_data: Vec<&str> = info_parts
        .lines()
        .filter(|x| x.starts_with("@PJL COMMENT "))
        .collect();
    vec_data = vec_data
        .iter()
        .map(|x| x.strip_prefix("@PJL COMMENT ").unwrap())
        .collect();
    let mut date_code = String::new();
    let mut supported_langs = String::new();
    let mut supported_models = vec![];
    for i in vec_data {
        let (first, second) = i.split_once('=').unwrap();
        match first {
            "LOCVERSION" => (),
            "VERSION" => (),
            "DATECODE" => date_code = second.to_string(),
            "MODEL" => supported_models.push(second.to_string()),
            "LANGUAGEGROUP" => supported_langs = second.to_string(),
            _ => return Err(format!("Unexpected rfu comment value {first}")),
        };
    }
    Ok(UpdateData {
        supported_models,
        supported_langs,
        date_code,
    })
}
