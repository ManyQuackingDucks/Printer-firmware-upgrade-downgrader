pub(super) fn send_update(ip: &str, f: std::fs::File) -> Result<(), String> {
    let mut conn = FtpServer::connect(ip.trim().to_string() + ":21").map_err(|e| format!("Could not connect to printer. [{e}]"));
    conn.put("update.rfu", &mut f)
        .map_err(|e| format!("Could not send file to printer. Try again. [{e}]"))?;
    conn.quit()
        .map_err(|e| format!("Could not close connection succesfully. [{e}]"))?;
    Ok(())
}