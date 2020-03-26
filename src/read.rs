use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::sync::{Arc, Mutex};

pub fn read_loop(input_file: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !input_file.is_empty() {
        Box::new(BufReader::new(File::open(input_file)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) 
        {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        Vec::from(&buffer[..num_read]);
    }

    let mut quit = quit.lock().unwrap(); 
    *quit = true;
    Ok(())
}
