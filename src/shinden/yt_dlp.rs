use std::process::Command;

pub fn download_m3u8(url: &str, output: &str) -> std::io::Result<bool> {
    let status = Command::new("yt-dlp")
        .args(["--concurrent-fragments", "16"])  
        //.args(["--add-header", &format!("Referer:{referer}")])
        .args(["--user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0"])
        .args(["--hls-prefer-native"])          
        .args(["--no-playlist"])
        .args(["--remux-video", "mp4"])            
        .args(["-o", output])
        .arg(url)
        .status()?;

    Ok(status.success())
}

fn drive_id(url: &str) -> Option<&str> {
    // .../file/d/<ID>/preview albo .../open?id=<ID>
    if let Some(rest) = url.split("/file/d/").nth(1) {
        return rest.split(&['/', '?'][..]).next();
    }
    url.split("id=").nth(1)?.split('&').next()
}

pub fn download_drive(url: &str, output: &str) -> std::io::Result<bool> {
    let Some(id) = drive_id(url) else {
        return Ok(false);
    };
    let view = format!("https://drive.google.com/file/d/{id}/view");

    let status = Command::new("yt-dlp")
        .args(["-o", output])
        .arg(view)
        .status()?;
    Ok(status.success())
}

pub fn download_cda(url: &str, output: &str) -> std::io::Result<bool> {
    let status = Command::new("yt-dlp")
    .args(["-o", output])
    .arg(url)
    .status()?;
    Ok(status.success())
}