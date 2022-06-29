#![allow(unused_variables)]
#![allow(dead_code)]

use colour::*;
use std::path::Path;
use std::{fs, io::Write};

fn cls() {
    print!("{esc}c", esc = 27 as char);
}

//Made by Xanthus
//Check out my other works at https://github.com/Xanthus58
//Email me at 'Xanthus58@protonmail.com'
//You can see more information on my website https://xanthus58.github.io/Xanthus58/

fn main() {
    cls();
    println!("Made by Xanthus58");
    println!("You can see more information on my website https://xanthus58.github.io/Xanthus58/");
    println!("If you notice any files not being sorted make an issue here https://github.com/Xanthus58/automatic_file_sorter/issues");
    println!("\n-Logs-");
    loop {
        let entries = fs::read_dir("./").unwrap();
        for entry in entries {
            let path = entry.unwrap().path();
            let file_name = match path.file_name() {
                Some(file_name) => file_name,
                None => continue,
            };
            let download_dir = match path.extension() {
                // Pictures
                Some(ext) if ext == "jpg" => "Pictures",
                Some(ext) if ext == "jpeg" => "Pictures",
                Some(ext) if ext == "png" => "Pictures",
                Some(ext) if ext == "psd" => "Pictures",
                Some(ext) if ext == "svg" => "Pictures",
                Some(ext) if ext == "ai" => "Pictures",
                // Videos
                Some(ext) if ext == "mp4" => "Videos",
                Some(ext) if ext == "mkv" => "Videos",
                Some(ext) if ext == "avi" => "Videos",
                Some(ext) if ext == "webm" => "Videos",
                Some(ext) if ext == "mov" => "Videos",
                // Music
                Some(ext) if ext == "mp3" => "Audio",
                Some(ext) if ext == "ogg" => "Audio",
                Some(ext) if ext == "wma" => "Audio",
                // Gifs
                Some(ext) if ext == "gif" => "Gifs",
                Some(ext) if ext == "apng" => "Gifs",
                // Files
                Some(ext) if ext == "zip" => "Files",
                Some(ext) if ext == "rar" => "Files",
                Some(ext) if ext == "tar" => "Files",
                Some(ext) if ext == "7z" => "Files",
                Some(ext) if ext == "cfg" => "Files",
                // Doccuments
                Some(ext) if ext == "txt" => "Documents",
                Some(ext) if ext == "pptx" => "Documents",
                Some(ext) if ext == "clsx" => "Documents",
                // Torrents
                Some(ext) if ext == "torrent" => "Torrents",
                // System Images
                Some(ext) if ext == "iso" => "SystemImages",
                // Fonts
                Some(ext) if ext == "fnt" => "Fonts",
                Some(ext) if ext == "fon" => "Fonts",
                Some(ext) if ext == "otf" => "Fonts",
                Some(ext) if ext == "ttf" => "Fonts",
                // Programming
                Some(ext) if ext == "py" => "Programming/Python",
                Some(ext) if ext == "rs" => "Programming/Rust",
                Some(ext) if ext == "js" => "Programming/JavaScript",
                Some(ext) if ext == "jar" => "Programming/Java",
                Some(ext) if ext == "html" => "Programming/HTML",
                Some(ext) if ext == "c" => "Programming/C",
                Some(ext) if ext == "cpp" => "Programming/C++",
                Some(ext) if ext == "cs" => "Programming/C#",
                Some(ext) if ext == "go" => "Programming/Go",
                Some(ext) if ext == "swift" => "Programming/Swift",
                Some(ext) if ext == "php" => "Programming/PHP",
                Some(ext) if ext == "r" => "Programming/R",
                // Applications
                Some(ext) if ext == "msi" => "Applications",
                Some(ext) if ext == "apk" => "Applications",
                Some(ext) if ext == "exe" => "Applications",
                _ => continue,
            };
            let download_dir = Path::new(download_dir);
            fs::create_dir_all(download_dir).unwrap();
            fs::rename(&path, download_dir.join(file_name)).unwrap();

            let log_dir = "sorter_logs";
            let log_file = "/logs.txt";
            fs::create_dir_all(log_dir).unwrap();

            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("sorter_logs/logs.txt")
                .expect("create failed");

            file.write_all(format!("{:?}", file_name).as_bytes())
                .expect("write failed");
            file.write_all(" Moved to ".as_bytes())
                .expect("write failed")
            file.write_all(format!("{:?}\n", download_dir.display()).as_bytes())
                .expect("write failed");

            print!("Name: ");
            green!("{} ", path.display());
            print!("Moved to ");
            red!("{}\n", download_dir.display());
        }
    }
}
