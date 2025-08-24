// hawkwood-corporation/src/systems/assets_system.rs
use crate::prelude::*;
use std::{fs, path::PathBuf};

impl<T> Site<T> {
    
    pub fn copy_assets(&self) {
        let assets_path = PathBuf::from("./assets");
        
        // Check if assets folder exists
        if !assets_path.exists() {
            println!("No assets folder found, skipping asset copying");
            return;
        }
        
        let mut output_path = PathBuf::from(".");
        output_path.push(&self.settings.output_folder);
        
        println!("Copying assets from {:?} to {:?}", assets_path, output_path);
        
        // Copy the contents of assets directory to output root
        if let Err(e) = copy_dir_contents(&assets_path, &output_path) {
            eprintln!("Failed to copy assets: {}", e);
        }/* else {
            println!("Assets copied successfully!");
        }*/
    }
}

fn copy_dir_contents(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    // Create destination directory
    fs::create_dir_all(dst)?;
    
    // Read the source directory and copy its contents
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            // Recursively copy subdirectories
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            // Copy files only if they don't exist or are newer
            if should_copy_file(&src_path, &dst_path)? {
                fs::copy(&src_path, &dst_path)?;
                println!("Copied: {:?} -> {:?}", src_path, dst_path);
            } else {
                println!("Skipped (up to date): {:?}", src_path);
            }
        }
    }
    
    Ok(())
}

fn should_copy_file(src: &PathBuf, dst: &PathBuf) -> std::io::Result<bool> {
    // If destination doesn't exist, always copy
    if !dst.exists() {
        return Ok(true);
    }
    
    // Compare file sizes first (fastest check)
    let src_metadata = fs::metadata(src)?;
    let dst_metadata = fs::metadata(dst)?;
    
    // If sizes are different, definitely need to copy
    if src_metadata.len() != dst_metadata.len() {
        return Ok(true);
    }
    
    // If sizes are same, check modification times
    let src_modified = src_metadata.modified()?;
    let dst_modified = dst_metadata.modified()?;
    
    // Copy if source is newer than destination
    Ok(src_modified > dst_modified)
}

fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    // Create destination directory
    fs::create_dir_all(dst)?;
    
    // Read the source directory
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            // Recursively copy subdirectories
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            // Copy files only if they don't exist or are newer
            if should_copy_file(&src_path, &dst_path)? {
                fs::copy(&src_path, &dst_path)?;
                println!("Copied: {:?} -> {:?}", src_path, dst_path);
            } else {
                println!("Skipped (up to date): {:?}", src_path);
            }
        }
    }
    
    Ok(())
}