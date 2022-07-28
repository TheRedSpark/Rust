use std::path::Path;
use std::{thread, time};
use std::sync::mpsc::{self, TryRecvError};

extern crate fs_extra;

use fs_extra::dir::*;
use fs_extra::error::*;


fn example_copy() -> Result<()> {
    let path_from = Path::new("Arbeitsordner");
    let path_to = path_from.join("Kopieziel");
    let test_folder = path_from.join("Sourcefolder");
    let dir = test_folder.join("dir");
    let sub = dir.join("sub");
    let file1 = dir.join("file1.txt");
    let file2 = sub.join("file2.txt");

    create_all(&sub, true)?;
    create_all(&path_to, true)?;
    //fs_extra::file::write_all(&file1, "content1")?;
    //fs_extra::file::write_all(&file2, "content2")?;

    assert!(dir.exists());
    assert!(sub.exists());
    //assert!(file1.exists());
    //assert!(file2.exists());


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
        copy(&test_folder, &path_to, &options).unwrap();
    });

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