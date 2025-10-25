use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use memmap2::Mmap;


struct Filename {
    filepath: String,
}

impl Filename {
    fn new(filepath: &str) -> Filename {
        Filename {
            filepath: filepath.to_string(),
        }
    }

    fn readline(&self) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        let file = File::open(&self.filepath)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let _line = line?;
        }

        println!("Line-by-line read in {:?}", start.elapsed());
        Ok(())
    }

    fn readall(&self) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        let file = File::open(&self.filepath)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let content = std::str::from_utf8(&mmap)?;
        let _lines = content.lines().count();

        println!("Memory-mapped read in {:?}", start.elapsed());
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = Filename::new("large.csv");
    f.readline()?;
    f.readall()?;
    Ok(())
}
