/*
    Touch utility implementation.
    Copyright 2017 Sam Saint-Pettersen.

    Released under the MIT License.
*/

extern crate clioptions;
extern crate filetime;
use clioptions::CliOptions;
use filetime::FileTime;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::exit;

fn display_error(program: &str, err: &str) {
    println!("touch: {}.", err);
    println!("Try '{} -h | --help' for more information.", program);
    exit(-1);
}

fn display_version() {
    println!("touch v0.1.0.");
    exit(0);
}

fn display_help(program: &str) {
    println!("Touch utility implementation.");
    println!("Copyright 2017 Sam Saint-Pettersen.");
    println!("\nReleased under the MIT License.\n");
    println!("Usage: {} <file> [options] \n", program);
    println!("Options:\n");
    println!("-c | --no-create: Do not create file if it does not exist.");
    println!("-a | --access: Change the access time only.");
    println!("-m | --modification: Change the modification time only.");
    println!("-d | --date <date string>: Use ISO 8601 string for timestamp.");
    println!("-r | --reference <ref_file>: Use reference file's time instead of current time.");
    exit(0);
}

fn touch(program: &str, file: &str, create: bool, access: bool, modify: bool, rfile: String) {
    let touchf = "__touch_file__";
    let p = Path::new(file);
    if !p.exists() && create {
        let _ = File::create(file);
    }
    if p.exists() {
        let _ = File::create(touchf);
        let _ = File::open(file);
        let dt = fs::metadata(touchf).unwrap();
        let df = fs::metadata(file).unwrap();
        let mut tatime = FileTime::from_last_access_time(&dt);
        let mut tmtime = FileTime::from_last_modification_time(&dt);
        if !rfile.is_empty() {
            let rp = Path::new(&rfile);
            if rp.exists() {
                let drf = fs::metadata(&rfile).unwrap();
                tatime = FileTime::from_last_access_time(&drf);
                tmtime = FileTime::from_last_modification_time(&drf);
            } else {
                let _ = fs::remove_file(touchf);
                display_error(program, "cannot access reference file");
            }
        }
        let fatime = FileTime::from_last_access_time(&df);
        let fmtime = FileTime::from_last_modification_time(&df);
        if access && !modify {
            let _ = filetime::set_file_times(file, tatime, fmtime);
        } else if modify && !access {
            let _ = filetime::set_file_times(file, fatime, tmtime);
        } else {
            let _ = filetime::set_file_times(file, tatime, tmtime);
        }
        let _ = fs::remove_file(touchf);
    }
    exit(0);
}

fn main() {
    let cli = CliOptions::new("touch");
    let program = cli.get_program();

    let file = cli.next_argument(0);
    let mut rfile = String::new();
    // let mut date = String::new();
    let mut create = true;
    let mut access = true;
    let mut modify = true;

    if cli.get_num() == 2 {
        if file.trim() == "-h" || file.trim() == "--help" {
            display_help(&program);
        } else if file.trim() == "-v" || file.trim() == "--version" {
            display_version();
        }
    } else if cli.get_num() > 2 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-c" | "--no-create" => create = false,
                "-a" | "--access" => modify = false,
                "-m" | "--modification" => access = false,
                "-d" | "--date" => display_error(&program, "-d | --date not yet implemented"),
                "-r" | "--reference" => rfile = cli.next_argument(i),
                _ => continue
            }
        }
    } else {
        display_error(&program, "missing file operand");
    }
    touch(&program, &file, create, access, modify, rfile);
}
