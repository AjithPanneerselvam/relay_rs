use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::sync::{Arc, Mutex};

pub fn write_loop(output_file: &str, quit: Arc<Mutex<bool>>) -> Result<()> {

    let mut writer: Box<dyn Write> = if !output_file.is_empty() {
        Box::new(BufWriter::new(File::open(output_file)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // todo: receive vector from stats thread 
        let buffer: Vec<u8> = Vec::new(); 
        {
            let quit = quit.lock().unwrap(); 
            if *quit {
                break;
            }
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // stop the program cleanly
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}
