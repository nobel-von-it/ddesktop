use core::panic;
use std::io::Write;

fn main() {
    println!("enter program name: ");

    let mut program_name = String::new();
    std::io::stdin().read_line(&mut program_name).map_err(|err| {
        eprintln!("io error {err}")
    }).unwrap();

    create_file(program_name.trim());

    println!("done")
}
fn find_files(start: &str, filename: String) -> Option<String> {
    for entry in walkdir::WalkDir::new(start) {
        let entry = entry.unwrap();
        if let Some(filename_os) = entry.file_name().to_str() {
            if filename_os == filename {
                return Some(entry.path().to_str().unwrap().to_string())
            }
        }
    }
    None
}
fn create_file(program_name: &str) {
    let username = whoami::username();
    let start_path = format!("/home/{username}/");
    let desktop_name = program_name.to_lowercase();
    let mut desktop = std::fs::File::create(
        format!("{start_path}{name}.desktop", name = &desktop_name)
    ).map_err(|err| {
            eprintln!("file create error {err}")
        }).unwrap();
    let exec = find_files(&start_path, format!("{desktop_name}.sh")).unwrap_or_else(|| panic!("exec find error"));
    let icon = find_files(&start_path, format!("{desktop_name}.png")).unwrap_or_else(|| panic!("exec find error"));

    let _ = writeln!(desktop, "[Desktop Entry]");
    let _ = writeln!(desktop, "Name={program_name}");
    let _ = writeln!(desktop, "Type=Application");
    let _ = writeln!(desktop, "Terminal=false");
    let _ = writeln!(desktop, "Exec={exec}");
    let _ = writeln!(desktop, "Icon={icon}");
}
