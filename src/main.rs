use clap::App;
use shadow_rs::shadow;
use std::process::exit;
shadow!(build);

fn main() {
    App::new(build::PROJECT_NAME)
        .version(build::PKG_VERSION)
        .author(format!("Author: {} <{}>", build::COMMIT_AUTHOR, build::COMMIT_EMAIL).as_str())
        .get_matches();
    match std::env::current_exe() {
        Ok(pb) => {
            match std::env::consts::FAMILY {
                "windows" => match pb.read_link() {
                    Ok(pb3) => print!("{}", pb3.parent().unwrap().display()),
                    Err(_) => print!("{}", pb.parent().unwrap().display()),
                },
                _ => match pb.read_link() {
                    Ok(pb2) => match pb2.canonicalize() {
                        Ok(pb3) => print!("{}", pb3.parent().unwrap().display()),

                        Err(_) => exit(-2),
                    },
                    Err(_) => match pb.canonicalize() {
                        Ok(pb3) => print!("{}", pb3.parent().unwrap().display()),

                        Err(_) => exit(-2),
                    },
                },
            }
        }
        Err(_) => exit(-1),
    }
}
