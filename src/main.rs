use anyhow::Result;
use image::imageops::FilterType;
use std::{env, fs, fs::File, path::PathBuf};
use thiserror::Error;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let (path, npath, l, w, filter) = (
        args[1].as_str(),
        args[2].as_str(),
        args[3].parse::<u32>()?,
        args[4].parse::<u32>()?,
        args.get(5).map(|a| a.as_str()),
    );

    match get_file_paths(&path, filter) {
        Ok(file_paths) => resize(&file_paths, (l, w), &npath),
        Err(err) => Err(err),
    }
}

fn get_file_paths(path: &str, filter: Option<&str>) -> Result<Vec<PathBuf>> {
    let dir_entries = fs::read_dir(path)?;
    let file_paths = dir_entries
        .filter_map(|file| {
            let path = file.ok()?.path();
            let file_name = path.file_name()?.to_str()?;
            match filter {
                Some(filter) => match &file_name != &filter {
                    true => Some(path),
                    false => None,
                },
                None => Some(path),
            }
        })
        .collect();

    Ok(file_paths)
}

fn resize(file_paths: &Vec<PathBuf>, (length, width): (u32, u32), path: &str) -> Result<()> {
    for file in file_paths {
        use image::ImageFormat::{Jpeg, Png};
        use ResizeError::*;

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

#[derive(Debug, Error)]
enum ResizeError {
    #[error("Invalid String")]
    InvalidStr,
    #[error("Invalid format found (only Jpeg and Png allowed)")]
    InvalidFormat,
}
