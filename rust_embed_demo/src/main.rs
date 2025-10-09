use mime_guess::from_path;
use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "static"]
struct Asset;

fn read_file(file_path: &str) {
    if let Some(file) = Asset::get(file_path) {
        let bytes = file.data.as_ref();

        println!("File size: {} bytes", bytes.len());
    }
}

fn iter_folder() {
    for filename in Asset::iter() {
        println!("Embedded file: {}", filename.as_ref());
    }
}

fn get_mine(file_path: &str) {
    if let Some(file) = Asset::get(file_path) {
        let mime = from_path(file_path).first_or_octet_stream();
        println!("MIME: {}", mime);
    }
}

fn main() {
    if let Some(file) = Asset::get("index.html") {
        let content = std::str::from_utf8(file.data.as_ref()).unwrap();
        println!("index.html:\n{}", content);
    } else {
        println!("index.html not found");
    }

    read_file("style.css");
    iter_folder();

    get_mine("index.html");
}
