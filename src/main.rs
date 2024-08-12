use core::panic;
use std::{fmt::format, io::Write};

fn main() {
    println!("enter program name: ");

    let mut program_name = String::new();
    std::io::stdin()
        .read_line(&mut program_name)
        .map_err(|err| eprintln!("io error {err}"))
        .unwrap();

    create_file(program_name.trim());

    println!("done")
}
fn find_files(start: &str, filename: String) -> Option<String> {
    for entry in walkdir::WalkDir::new(start) {
        let entry = entry.unwrap();
        if let Some(filename_os) = entry.file_name().to_str() {
            if filename_os == filename {
                return Some(entry.path().to_str().unwrap().to_string());
            }
        }
    }
    None
}
fn create_file(program_name: &str) {
    let username = whoami::username();
    let start_path = format!("/home/{username}/");
    let desktop_name = program_name.to_lowercase();
    let mut desktop =
        std::fs::File::create(format!("{start_path}{name}.desktop", name = &desktop_name))
            .map_err(|err| eprintln!("file create error {err}"))
            .unwrap();

    let sh = format!("{desktop_name}.sh");
    let png = format!("{desktop_name}.png");
    let exec = find_files(&start_path, sh.clone()).unwrap_or(
        find_files("/run/media/", sh.clone()).unwrap_or_else(|| panic!("cannot find exec file")),
    );
    let icon = find_files(&start_path, png.clone()).unwrap_or(
        find_files("/run/media/", png.clone()).unwrap_or_else(|| panic!("cannot find icon file")),
    );

    let _ = writeln!(desktop, "[Desktop Entry]");
    let _ = writeln!(desktop, "Name={program_name}");
    let _ = writeln!(desktop, "Type=Application");
    let _ = writeln!(desktop, "Terminal=false");
    let _ = writeln!(desktop, "Exec={exec}");
    let _ = writeln!(desktop, "Icon={icon}");
}
