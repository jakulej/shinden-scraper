use std::process::Command;

pub fn download_m3u8(url: &str, referer: &str, output: &str) -> std::io::Result<bool> {
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