use std::path::Path;
use std::{thread, time};
use std::sync::mpsc::{self, TryRecvError};

extern crate fs_extra;

use fs_extra::dir::*;
use fs_extra::error::*;


fn example_copy() -> Result<()> {
    let path_from = Path::new("R:/Backup");
    let path_to = Path::new("Eingangsordner");
    //let path_shadow = Path::new("Shadowordner");
    let file1 = Path::new("Output.txt");

    create_all(&path_from, false)?;
    create_all(&path_to, false)?;
    //create_all(&path_shadow, false)?;
    let content = "Start";
    fs_extra::file::write_all(&file1, content)?;


    let options = CopyOptions {
        buffer_size: 1,
        ..Default::default()
    };
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let handler = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            thread::sleep(time::Duration::from_millis(500));
        };
        copy(&path_from, &path_to, &options).unwrap();
        //copy(&path_from, &path_shadow, &options).unwrap();
    });
    //fs_extra::file::write_all(&file1, "Finish")?;


    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                println!("{} of {} bytes",
                         process_info.copied_bytes,
                         process_info.total_bytes);
            }
            Err(TryRecvError::Disconnected) => {
                println!("finished");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
    Ok(())
}


pub(crate) fn main() {
    example_copy();
}