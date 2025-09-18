#![no_std]
#![no_main]

extern crate alloc;

use alloc::{vec::Vec, boxed::Box};
use alloc::vec;

use log::info;
use uefi::{boot::LoadImageSource, prelude::*, println, proto::media::{file::{Directory, File, FileAttribute, FileHandle, FileInfo, FileMode, FileType, RegularFile}, fs::SimpleFileSystem}};

use crate::allocator::UefiAllocator;

mod allocator;

#[global_allocator]
static ALLOCATOR: UefiAllocator = UefiAllocator;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Bootloader initialized.");
    println!("Bootloader initialized.");
    let image_handle = boot::image_handle();

    let mut binding = boot::get_image_file_system(image_handle)
        .unwrap();
    let simple_filesystem_pointer: &mut SimpleFileSystem = binding.get_mut().unwrap();

    let simple_filesystem: &mut SimpleFileSystem = &mut *simple_filesystem_pointer;

    let mut root: Directory = simple_filesystem.open_volume().unwrap();

    println!("Defined root: {:?}", root);
    let kernel_handle: FileHandle = root.open(
        cstr16!(r#"efi\\boot\\hakurei.efi"#), 
        FileMode::Read,
        FileAttribute::empty()
    ).unwrap();

    println!("Defined kernel handle: {:?}", kernel_handle);

    
    let mut kernel_file: RegularFile = match kernel_handle.into_type().unwrap() {
        FileType::Regular(f) => f,
        FileType::Dir(_) => panic!("Expected a regular file"),
    };

    println!("Found kernel file: {:?}", kernel_file);

    let kernel_info: Box<FileInfo>  = kernel_file.get_boxed_info().expect("Critical failure getting kernel info.");
    println!("Read kernel info: {:?}", kernel_info);
    let kernel_size = kernel_info.file_size() as usize; 
    let mut kernel_buffer: Vec<u8> = vec![0u8; kernel_size];

    println!("Reading kernel into buffer...");
    let read = kernel_file.read(&mut kernel_buffer).expect("Unable to read kernel");
    println!("Kernel read: {:?} | {:?}", kernel_buffer, read);

    let load_source = LoadImageSource::FromBuffer {
        buffer: &kernel_buffer,
        file_path: None,
    };

    println!("Defined image source: {:?}", load_source);
    println!("Loading kernel...");

    let kernel_image_handle = boot::load_image(image_handle, load_source).unwrap();
    println!("Bootloader finished.");
    boot::start_image(kernel_image_handle).unwrap();
    boot::stall(1_000_000);


    Status::SUCCESS
}
