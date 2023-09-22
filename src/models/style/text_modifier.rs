use core::fmt;

use bitflags::bitflags;



bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Modifier: u16 {
        const BOLD =         0b0000000000000001;
        const DIM =          0b0000000000000010;
        const ITALIC =       0b0000000000000100;
        const UNDERLINE =    0b0000000000001000;
        const SLOW_BLINK =   0b0000000000010000;
        const RAPID_BLINK =  0b0000000000100000;
        const REVERSED =     0b0000000001000000;
        const HIDDEN =       0b0000000010000000;
        const CROSSED_OUT =  0b0000000100000000;
    }


}

impl fmt::Debug for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "NONE");
        }
        fmt::Debug::fmt(&self.0, f)
    }

}
