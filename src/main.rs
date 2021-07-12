use std::process::exit;

fn main() {
    //std::env::consts::OS;
    match std::env::current_exe() {
        Ok(pb) => {
            // pb.symlink_metadata()
            // println!("{}", pb.display());
            //std::fs::read_link(pb);
            match pb.read_link() {
                Ok(pb2) => match pb2.canonicalize() {
                    Ok(pb3) => {
                        print!("{}", pb3.display())
                    }
                    Err(_) => exit(-2),
                },
                Err(_) => match pb.canonicalize() {
                    Ok(pb3) => {
                        print!("{}", pb3.display())
                    }
                    Err(_) => exit(-2),
                },
            }
        }
        Err(_) => exit(-1),
    }
}
