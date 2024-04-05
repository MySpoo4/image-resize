use std::{
    error::Error, 
    fs::File,
    path::PathBuf,
    fs, fmt, env, 
};
use image::imageops::FilterType;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (path, npath, l, w, filter) = (
        args[1].as_str(), 
        args[2].as_str(),
        args[3].parse::<u32>()?,
        args[4].parse::<u32>()?,
        args.get(5).map(|a| a.as_str()),
    );

    match get_file_paths(&path, filter) {
        Ok(file_paths) => {
            resize(&file_paths, (l, w), &npath)
        },
        Err(err) => Err(err),
    }
}

fn get_file_paths(path: &str, filter: Option<&str>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir_entries = fs::read_dir(path)?;
    let file_paths = dir_entries
        .filter_map(|file| {
            let path = file.ok()?.path();
            let file_name = path
                .file_name()?
                .to_str()?;
            match filter {
                Some(filter) => {
                    match &file_name != &filter {
                        true => Some(path),
                        false => None,
                    }
                },
                None => Some(path),
            }
        })
        .collect();

    Ok(file_paths)
}

#[derive(Debug)]
enum ResizeError {
    InvalidStr,
    InvalidFormat,
}

impl Error for ResizeError {}

impl fmt::Display for ResizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ResizeError::*;

        match self {
            InvalidStr => write!(f, "Invalid string parse"),
            InvalidFormat => write!(f, "Invalid format. Only Jpeg & Png allowed"),
        }
    }
}

fn resize(
    file_paths: &Vec<PathBuf>, 
    (length, width): (u32, u32),
    path: &str,
) -> Result<(), Box<dyn Error>> {

    for file in file_paths {
        use ResizeError::*;
        use image::ImageFormat::{Jpeg, Png};

        let file_name = match file.file_name() {
            Some(file_name) => file_name.to_str().ok_or(InvalidStr)?,
            None => continue,
        };
        let file_format = file
            .extension()
            .ok_or(InvalidFormat)?
            .to_str()
            .ok_or(InvalidStr)?;

        let img_format = match file_format {
            "jpg" => Ok(Jpeg),
            "png" => Ok(Png),
            _ => Err(InvalidFormat),
        }?;

        let img = image::open(&file)?;
        let new_img = img.resize(length, width, FilterType::Triangle);

        let mut file = File::create(format!("{path}/{file_name}"))?;
        new_img.write_to(&mut file, img_format)?;
    }

    Ok(())
}
