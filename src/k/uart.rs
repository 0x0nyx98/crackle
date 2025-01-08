use core::{fmt::{Error, Write}, str};

pub struct Uart {
    base: usize,
    unit: () // dont construct
}

impl Uart {
    pub unsafe fn new_handle_no_init(base_addr: usize) -> Uart {
        Uart {base: base_addr, unit: ()}
    }

    pub fn init(base_addr: usize) -> Uart {
        let base_ptr = base_addr as *mut u8;
        
        unsafe {
            let lcr = (1 << 0) | (1 << 1);
            base_ptr.add(3).write_volatile(lcr);
            base_ptr.add(2).write_volatile(1);
            base_ptr.add(1).write_volatile(1);

            let divisor: u16 = 592;
            let divisor_least: u8 = (divisor & 0xff) as u8;
            let divisor_most:  u8 = (divisor >> 8) as u8;

            base_ptr.add(3).write_volatile(lcr | 1 << 7);
            base_ptr.add(0).write_volatile(divisor_least);
            base_ptr.add(1).write_volatile(divisor_most);
            base_ptr.add(3).write_volatile(lcr);
        }

        Uart {base: base_addr, unit: ()}
    }

    pub fn listen(&self) -> Option<u8> {
        let ptr = self.base as *mut u8;

		unsafe {
			if ptr.add(5).read_volatile() & 1 == 0 {
				None
			} else {
				Some(ptr.add(0).read_volatile())
			}
		}
    }

    pub fn speak(&mut self, c: char) {
        let ptr = self.base as *mut u8;

        unsafe { ptr.add(0).write_volatile(c as u8); }
    }

    pub fn speak_full(&mut self, s: &str) {
        let ptr = self.base as *mut u8;

        unsafe { 
            for c in s.chars() {
                ptr.add(0).write_volatile(c as u8); 
            }
        }
    }
}

impl Write for Uart {
	fn write_str(&mut self, s: &str) -> Result<(), Error> {
		Ok(self.speak_full(s))
	}
}

