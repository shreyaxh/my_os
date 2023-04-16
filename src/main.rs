#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}

// cd C:/Users/shrey/OneDrive/Desktop/myOS/my_os
// qemu-system-x86_64 -drive format=raw,file=target/x86_64-my_os/debug/bootimage-my_os.bin
// qemu-system-x86_64 -drive format=raw,file=C:/Users/shrey/OneDrive/Desktop/myOS/my_os/target/x86_64-my_os/debug/deps/bootimage-my_os-69cc58abd2710cdc.bin -device isa-debug-exit,iobase=0xf4,iosize=0x04
// 1.
// pacman -S git && pacman -S mingw-w64-x86_64-toolchain

// 2.
// open msys2 and type:
// curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

// 2.
// choose custom:
// for Host triples identify enter:
// x86_64-pc-windows-gnu

// 3.
// default

// 4.
// add this line to ~/.bashrc
// export PATH="/c/Users/shrey/.cargo/bin:$PATH"
//
// 5.
// rustup override set nightly
