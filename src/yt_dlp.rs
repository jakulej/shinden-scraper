use std::process::Command;

pub fn download_m3u8(url: &str, referer: &str, output: &str) -> std::io::Result<bool> {
    let status = Command::new("yt-dlp")
        .args(["--concurrent-fragments", "16"])   // równoległe segmenty
        .args(["--add-header", &format!("Referer:{referer}")])
        .args(["--user-agent", "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0"])
        .args(["--hls-prefer-native"])             // natywny downloader, nie ffmpeg
        .args(["--no-playlist"])
        .args(["--remux-video", "mp4"])            // wymaga ffmpeg
        .args(["-o", output])
        .arg(url)
        .status()?; // wyjście yt-dlp leci prosto do terminala

    Ok(status.success())
}