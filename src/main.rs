#![no_std]
#![no_main]

extern crate alloc;

use alloc::{vec::Vec, boxed::Box};

use uefi::{boot::LoadImageSource, prelude::*, println, proto::media::{file::{Directory, File, FileAttribute, FileHandle, FileInfo, FileMode, FileType, RegularFile}, fs::SimpleFileSystem}};

#[entry]
fn efi_main() -> Status {
    let image_handle = boot::image_handle();

    let mut binding = boot::get_image_file_system(image_handle)
        .unwrap();
    let simple_filesystem_pointer: &mut SimpleFileSystem = binding.get_mut().unwrap();

    let simple_filesystem: &mut SimpleFileSystem = &mut *simple_filesystem_pointer;

    let mut root: Directory = simple_filesystem.open_volume().unwrap();

    let kernel_handle: FileHandle = root.open(
        cstr16!(r#"EFI\\HAKUREI\\kernel.efi"#), 
        FileMode::Read,
        FileAttribute::empty()
    ).unwrap();

    
    let mut kernel_file: RegularFile = match kernel_handle.into_type().unwrap() {
        FileType::Regular(f) => f,
        FileType::Dir(_) => panic!("Expected a regular file"),
    };

    let kernel_info: Box<FileInfo>  = kernel_file.get_boxed_info().unwrap();
    let kernel_size = kernel_info.file_size() as usize; 
    let mut kernel_buffer: Vec<u8> = Vec::with_capacity(kernel_size);
    kernel_file.read(&mut kernel_buffer).unwrap();

    let load_source = LoadImageSource::FromBuffer {
        buffer: &kernel_buffer,
        file_path: None,
    };

    let kernel_image_handle = boot::load_image(image_handle, load_source).unwrap();
    boot::start_image(kernel_image_handle).unwrap();
    boot::stall(10_000_000);


    Status::SUCCESS
}
