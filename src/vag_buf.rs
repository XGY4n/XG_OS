//vag_buf.rs
use volatile::Volatile;
use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
/*
    #[repr(transparent)] 是一个属性，它用于指定结构体的表示方式。
    它告诉 Rust 编译器将结构体表示为其唯一的非零大小字段。这对于创建安全的包装类型非常有用，因为它可以保证包装类型在 ABI 层面与被包装类型相同。 
*/
struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))//10101111 << 4 = 11110000 
    }
}
/*
    impl ColorCode 表示我们要为 ColorCode 类型定义关联函数。接下来的代码块中定义了一个名为 new 的关联函数，
    它是 ColorCode 类型的一个构造函数。它接受两个参数：前景色和背景色。这两个参数都是 Color 类型的枚举变量。
    函数使用位运算符将背景色和前景色组合成一个颜色代码，并返回一个新的 ColorCode 结构体。
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],

    /*
        在 Rust 中，你可以使用 [T; N] 语法来定义一个包含 N 个 T 类型元素的数组。
        例如，[ScreenChar; BUFFER_WIDTH] 表示一个包含 BUFFER_WIDTH 个 ScreenChar 结构体的数组。
        在这段代码中，chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] 定义了一个名为 chars 的字段，
        它是一个二维数组，包含 BUFFER_HEIGHT * BUFFER_WIDTH 个 ScreenChar 结构体。
    */
}
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: & 'static mut Buffer,//全局静态指针
}
/* xxx::www
    usize 是 Rust 中的一个基本类型，它表示一个指针大小的无符号整数。它的大小取决于目标平台：在 32 位系统上，
    它的大小为 32 位；在 64 位系统上，它的大小为 64 位。
    usize 类型通常用于表示大小和索引。例如，在数组和切片中，
    索引必须是 usize 类型。当你需要表示一个与目标平台指针大小相关的值时，也应该使用 usize 类型。
*/
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte { // 匹配, 类似switch case?
            b'\n' => self.new_line(),//换行 new line
            byte => {
                if self.column_position >= BUFFER_WIDTH {//非换行符 检查是否触碰边界
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {// set ascii and color
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT{ //[1, BUFF)
            for col in 0..BUFFER_WIDTH{
                let charater = self.buffer.chars[row][col].read();//copy
                self.buffer.chars[row - 1][col].write(charater);//self move copy
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, row:usize){
        let blank = ScreenChar{
            ascii_character : b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                0x20..=0x7e | b'\n' => self.write_byte(byte),//语法糖 意思是match 范围在 [0x20 ,0x7e]之间的值 或者 \n
                // 不包含在上述范围之内的字节
                _ => self.write_byte(0xfe),
            }

        }
    }
}

impl fmt::Write for Writer{//一个fmt宏, 针对Write的, 可用用{}来格式化打印东西
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}



pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::LightGray),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
    write!(writer, "fmt write test {} and {}", "teststr", 0/1).unwrap();
    //UTF-8 编码的基本特点之一：如果一个字符占用多个字节，那么每个组成它的独立字节都不是有效的 ASCII 码字节
}


use lazy_static::lazy_static;
use spin::Mutex;//自旋互斥锁
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vag_buf::_print(format_args!($($arg)*)));
}

#[macro_export]
/*
$ 符号用于宏定义中，表示模式匹配和重复。
例如，在 print! 宏的定义中，$($arg:tt)* 表示匹配任意数量的 tt 类型的参数，
并将它们捕获到名为 arg 的变量中。在宏的右侧，$($arg)* 表示将捕获到的所有参数重复展开。

 */
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}


