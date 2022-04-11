pub mod tftp;

pub fn update() -> Result<(), String>{
    let stdin = std::io::stdin();
    println!("Please enter your printers address: ");
    let mut ip = String::new();
    stdin
        .read_line(&mut ip)
        .map_err(|e| format!("Could not read from stdin. [{e}]"))?;
    println!("Please select an update method\nFor tftp enter A\n For ipp enter B");
    let mut buffer = String::new();
    stdin
        .read_line(&mut buffer)
        .map_err(|e| format!("Could not read from stdin. [{e}]"))?;   
    match buffer.trim(){
        "A" => {
            let f = read_parse(&ip)?;
            tftp::send_update(&ip, f)
        }
    }
}

pub(super) fn read_parse(ip: &str) -> Result<std::fs::File, String>{
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
    let mut buf
     = [0; 10000];

    f.read(&mut buf)
        .map_err(|e| format!("Could not read from file {} [{e}]", &file_location))?;
    let update_data = parse_rfu_bytes(&buf)?;

    println!("Loaded file: {}\nSupported models:", &file_location);
    for i in update_data.supported_models {
        println!("{i}");
    }
    println!(
        "datecode: {}\nSupported langs: {}",
        update_data.date_code, update_data.supported_langs
    );
    Ok(f)
}

struct UpdateData {
    supported_models: Vec<String>,
    date_code: String,
    supported_langs: String,
}