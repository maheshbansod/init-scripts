use std::io::Write;
use std::{
    env,
    fs::{read_to_string, File},
    io,
    path::Path,
};

pub fn statement_exists_in_bashrc(stat: &str) -> bool {
    let bashrc_path = get_bashrc_path();
    let bashrc_path = Path::new(&bashrc_path);
    if let Ok(bashrc) = read_to_string(bashrc_path) {
        bashrc.contains(stat)
    } else {
        false
    }
}

fn get_bashrc_path() -> String {
    format!("{}/.bashrc", env::var("HOME").unwrap())
}

pub fn add_statement_to_bashrc(stat: &str) -> io::Result<()> {
    let bashrc_path = get_bashrc_path();
    let bashrc_path = Path::new(&bashrc_path);
    let mut file = File::options().append(true).open(bashrc_path)?;
    writeln!(&mut file, "{}", stat)
}
