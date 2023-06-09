use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
/*
自旋锁并不会调用阻塞逻辑，而是在一个小的无限循环中反复尝试获得这个锁，
也因此会一直占用 CPU 时间，直到互斥锁被它的占用者释放。
 */
lazy_static!{
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args : ::core::fmt::Arguments){
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("print error");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    //concat 切分连接
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}