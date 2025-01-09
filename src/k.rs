                    use crate::*;
mod uart;           use uart::*;

pub use uart::Uart;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    let mut uart = Uart::init(0x1000_0000);
    uart.speak_full("test!");

    println!("testing!");

    loop {}
}

struct Mmio {

}

impl Mmio {
    unsafe fn write(addr: usize, off: usize, val: u8) {
        unsafe { (addr as *mut u8).add(off).write_volatile(val); }
    }

    unsafe fn read(addr: usize, off: usize) -> u8 {
        unsafe { (addr as *mut u8).add(off).read_volatile() }
    }
}

