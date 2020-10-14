use core::panic::PanicInfo;
use core::alloc::Layout;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n*****PANIC*****\nOh! There has some problem!\n{}", info);

    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort!");
}


#[lang = "oom"]
fn oom(layout: Layout) -> ! {
    panic!("Memory allocation of {} bytes failed", layout.size());
}


#[lang = "eh_personality"]
#[no_mangle]
fn eh_personality() -> ! {
    loop {}
}