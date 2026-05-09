use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::error::Error;

pub fn setupFileHandlingPractice() -> Result<(), Box<dyn Error>> {
    //openFile()?;
    copyFileBackwards()?;
    Ok(())
}

fn openFile() -> Result<(), Box<dyn Error>>{
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("example.txt")?;
    
    println!("Was able to open the file");
    readFile(&mut file)?;
    writeToFile(&mut file)?;
    readFile(&mut file)?;
    Ok(())
}

fn readFile(file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut contents)?;
    println!("Was able to read its contents: {contents}");
    Ok(())
}

fn writeToFile(file: &mut File) -> Result<(), Box<dyn Error>> {
    file.write(b"\nThis was written to the file")?;
    println!("Was able to write to the file");
    Ok(())
}

fn copyFileBackwards() -> Result<(), Box<dyn Error>>{
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut contents)?;
    contents = contents.chars().rev().collect::<String>();
    let mut newFile = File::create("newFile.txt")?;
    newFile.write_all(contents.as_bytes())?;
    println!("Original file contents:");
    readFile(&mut file)?;
    println!("New file contents: {contents}");
    Ok(())
}

