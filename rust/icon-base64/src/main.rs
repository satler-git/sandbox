use std::path::{Path, PathBuf};

// Use https://crates.io/crates/freedesktop-icons
// fn find_icon_path(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
//     if name.starts_with("/") {
//         Ok(name.into())
//     } else {
//         let mut pathes: Vec<PathBuf> = vec![dirs::home_dir()
//             .expect("Failed to get Home Dir")
//             .join(".icons")];
//
//         let mut icon_dirs = std::env::var("XDG_DATA_DIRS")?
//             .split(":")
//             .map(|s| format!("{s}/icons"))
//             .map(|s| PathBuf::from(s))
//             .collect();
//
//         pathes.append(&mut icon_dirs);
//
//         pathes.push("/usr/share/pixmaps".into());
//
//         todo!()
//     }
// }

fn load_image(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    use base64::{engine::general_purpose, Engine as _};
    use image::{ImageFormat, ImageReader};
    use std::io::Cursor;

    let img = ImageReader::open(path)?.decode()?;

    let mut buff = Cursor::new(vec![]);

    img.write_to(&mut buff, ImageFormat::Png)?;

    Ok(general_purpose::STANDARD.encode(&buff.get_ref()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
