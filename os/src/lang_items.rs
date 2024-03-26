use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler] // an outer attribute, tell compiler that we implement the core's empty panic() by the following panic()
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {} {} {}", 
            location.file(), 
            location.line(), 
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown(true)
}