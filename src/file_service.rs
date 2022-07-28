extern crate fs_extra;
use fs_extra::dir;
use fs_extra::error;
use fs_extra::file;
use fs_extra::dir::create;
//
//
//
//
pub(crate) fn main()  {
    println!("===========================");
    println!("File Service");
    println!("===========================");

    create("ausgangsfolder", false).expect("0001:Ordner existiert bereits");

}