#![no_std]
#![no_main]

use uefi::{prelude::*, println};
use tracing::info;
use uefi_raw::table::system::SystemTable;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();
    //tracing_subscriber::fmt().init();
    //info!("Hello World!");
    println!("Hello World!"); 
    boot::stall(10_000_000);
    Status::SUCCESS
}
