use crossbeam::Receiver;
use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write_loop(output_file: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !output_file.is_empty() {
        Box::new(BufWriter::new(File::create(output_file)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        let buffer = write_rx.recv().unwrap();
        if buffer.is_empty() {
            break;
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
