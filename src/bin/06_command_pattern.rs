use std::fs::File;
use std::io::{BufRead, BufReader, Seek, Write};



trait Command {
    fn execute(&self) -> std::io::Result<()>;
}


struct ReadFile {
    receiver: File,
}

impl ReadFile {
    fn new(receiver: File) -> Box<Self> {
        Box::new(Self { receiver })
    }
}

impl Command for ReadFile {
    fn execute(&self) -> std::io::Result<()> {
        println!("Reading file from the start");
        let mut reader = BufReader::new(&self.receiver);
        reader.seek(std::io::SeekFrom::Start(0))?;
        
        for (count, line) in reader.lines().enumerate() {
            println!("{:2}: {}", count +1, line?);
        }
        
        Ok(())
    }
}


struct WriteFile {
    content: String,
    receiver: File
}


impl WriteFile {
    fn new(content: String, receiver: File) -> Box<Self> {
        Box::new(Self { content, receiver })
    }
}


impl Command for WriteFile {
    fn execute(&self) -> std::io::Result<()> {
        println!("Writing new content to file");
        let mut writer = self.receiver.try_clone()?;
        
        writer.write_all(self.content.as_bytes())?;
        writer.flush()?;
        
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("file.txt")?;
    
    let commands: Vec<Box<dyn Command>> = vec![
        ReadFile::new(file.try_clone()?),
        WriteFile::new("file content\n".to_string(), file.try_clone()?),
        ReadFile::new(file.try_clone()?),
    ];
    
    for command in commands {
        command.execute()?;
    }
    
    Ok(())
}