#![no_std]
#![no_main]


use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

//By using the #[no_mangle] attribute we disable the name mangling to ensure that the Rust compiler really outputs a function with the name _start. Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE symbol to give every function an unique name. The attribute is required because we need to tell the name of the entry point function to the linker in the next step.
//We also have to mark the function as extern "C" to tell the compiler that it should use the C calling convention for this function (instead of the unspecified Rust calling convention). 
//The ! return type means that the function is diverging, i.e. not allowed to ever return
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}

//for compiling with linker arguments
//cargo rustc -- -C link-arg=-nostartfiles


