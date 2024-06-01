use std::{env, fs};

fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_IMAGE");
    let bios_path = env!("BIOS_IMAGE");
    let args: Vec<String> = env::args().collect();


    // choose whether to start the UEFI or BIOS image
    let uefi = if args.get(1).is_some() {
        true
    } else {
        false
    };

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}