use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn reader(path: Option<&Path>) -> io::Result<Box<dyn Read>> {
    Ok(path
        .map(File::open)
        .transpose()?
        .map(|fi| Box::new(fi) as Box<dyn Read>)
        .unwrap_or(Box::new(std::io::stdin())))
}
