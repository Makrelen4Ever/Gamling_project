pub enum Color {
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
    RESET,
}
pub fn switch_color(col: Color) {
    let output: &str;
    match col {
        Color::BLACK => output = "\x1B[30m",
        Color::RED => output = "\x1B[31m",
        Color::GREEN => output = "\x1B[32m",
        Color::YELLOW => output = "\x1B[33m",
        Color::BLUE => output = "\x1B[34m",
        Color::MAGENTA => output = "\x1B[35m",
        Color::CYAN => output = "\x1B[36m",
        Color::WHITE => output = "\x1B[37m",
        Color::RESET => output = "\x1B[39m",
    }

    print!("{output}");
}

pub fn clear_terminal() {
    print!("{esc}c", esc = 27 as char)
}
