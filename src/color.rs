use crate::dump::VisuMode;
use colored::*;

#[macro_export]
macro_rules! println_error {
    ($($arg:tt)*) => {{
        use colored::*;
        eprintln!("{}", format!($($arg)*).bright_red());
    }};
}

#[macro_export]
macro_rules! println_info {
    ($($arg:tt)*) => {{
        use colored::*;
        eprintln!("{}", format!($($arg)*).yellow());
    }};
}

#[cfg(windows)]
pub fn enable_ansi_support() {
    use windows_sys::Win32::System::Console::{
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE,
        SetConsoleMode,
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode = 0;
        if GetConsoleMode(handle, &mut mode) != 0 {
            SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

// Frame printing functions
const FRAME_R: u8 = 120;
const FRAME_G: u8 = 120;
const FRAME_B: u8 = 120;

pub fn print_frame_head(left_base_padding: usize, right_base_padding: usize) {
    // first line
    // print left corner
    print!("{}", format!("┌").truecolor(FRAME_R, FRAME_G, FRAME_B));

    // print the left padding
    for _i in 0..left_base_padding {
        print!("{}", format!("─").truecolor(FRAME_R, FRAME_G, FRAME_B));
    }

    // print the BASE stroke
    print!("{}", format!("────").truecolor(FRAME_R, FRAME_G, FRAME_B));

    // print the right padding
    for _i in 0..right_base_padding {
        print!("{}", format!("─").truecolor(FRAME_R, FRAME_G, FRAME_B));
    }

    // print the rest of the frame
    println!(
        "{}",
        format!(
            "┬────────────────────────────────┬────────────────────────────────┬──────────────────┐"
        )
        .truecolor(FRAME_R, FRAME_G, FRAME_B)
    );

    // middle line
    // print left wall
    print!("{}", format!("│").truecolor(FRAME_R, FRAME_G, FRAME_B));

    // print left padding
    for _i in 0..left_base_padding {
        print!(" ");
    }

    // print BASE
    print!("BASE");

    // print right padding
    for _i in 0..right_base_padding {
        print!(" ");
    }

    // print wall with one space
    print!("{}", format!("│ ").truecolor(FRAME_R, FRAME_G, FRAME_B));

    // print index
    for i in 0..16 {
        print!("{}", format!("{:02X} ", i).cyan());
        if i == 7 {
            print!("{} ", format!("│").truecolor(FRAME_R, FRAME_G, FRAME_B));
        } else if i != 15 {
            print!(" ");
        }
    }

    // print ASCII section
    println!(
        "{}      ASCII       {}",
        format!("│").truecolor(FRAME_R, FRAME_G, FRAME_B),
        format!("│").truecolor(FRAME_R, FRAME_G, FRAME_B)
    );

    // bottom line
    // print wall
    print!("{}", format!("├").truecolor(FRAME_R, FRAME_G, FRAME_B));

    // print the left padding
    for _i in 0..left_base_padding {
        print!("{}", format!("─").truecolor(FRAME_R, FRAME_G, FRAME_B));
    }

    // print the BASE stroke
    print!("{}", format!("────").truecolor(FRAME_R, FRAME_G, FRAME_B));

    // print the right padding
    for _i in 0..right_base_padding {
        print!("{}", format!("─").truecolor(FRAME_R, FRAME_G, FRAME_B));
    }

    // print the rest of the frame
    println!(
        "{}",
        format!(
            "┼────────────────────────────────┼────────────────────────────────┼──────────────────┤"
        )
        .truecolor(FRAME_R, FRAME_G, FRAME_B)
    );
}

pub fn print_frame_foot(left_base_padding: usize, right_base_padding: usize) {
    // print left corner
    print!("{}", format!("└").truecolor(FRAME_R, FRAME_G, FRAME_B));

    // print the left padding
    for _i in 0..left_base_padding {
        print!("{}", format!("─").truecolor(FRAME_R, FRAME_G, FRAME_B));
    }

    // print the BASE stroke
    print!("{}", format!("────").truecolor(FRAME_R, FRAME_G, FRAME_B));

    // print the right padding
    for _i in 0..right_base_padding {
        print!("{}", format!("─").truecolor(FRAME_R, FRAME_G, FRAME_B));
    }

    // print the rest of the frame
    println!(
        "{}",
        format!(
            "┴────────────────────────────────┴────────────────────────────────┴──────────────────┘"
        )
        .truecolor(FRAME_R, FRAME_G, FRAME_B)
    );
}

pub fn print_frame_part(str: impl AsRef<str>) {
    print!(
        "{}",
        format!("{}", str.as_ref()).truecolor(FRAME_R, FRAME_G, FRAME_B)
    );
}

pub fn print_base_addr(addr: u64, base_width: usize) {
    print!(
        "{}",
        format!("{addr:0width$X}", width = base_width).yellow()
    );
}

pub fn print_repeated(str: impl AsRef<str>, n: usize) {
    print!("{}", format!("{}", str.as_ref()).repeat(n));
}

pub fn print_dark(str: impl AsRef<str>) {
    const DARK_R: u8 = 64;
    const DARK_G: u8 = 64;
    const DARK_B: u8 = 64;
    print!(
        "{}",
        format!("{}", str.as_ref()).truecolor(DARK_R, DARK_G, DARK_B)
    );
}

pub fn print_byte(data: u8, visu_mode: VisuMode) {
    match visu_mode {
        VisuMode::Default => {
            print!("{data:02X} ");
        }
        VisuMode::AsciiGraphic => {
            if data.is_ascii_graphic() || data == b' ' {
                print!("{} ", format!("{data:02X}").bright_green());
            } else {
                print!("{} ", format!("{data:02X}").bright_red());
            }
        }
        VisuMode::HighlightZeros => {
            if data == 0 {
                print!("{} ", format!("{data:02X}").bright_magenta());
            } else {
                print!("{data:02X} ");
            }
        }
        VisuMode::HighBytes => {
            if data >= 0x80 {
                print!("{} ", format!("{data:02X}").bright_blue());
            } else {
                print!("{data:02X} ");
            }
        }
        VisuMode::ControlChars => {
            if data.is_ascii_control() {
                print!("{} ", format!("{data:02X}").yellow());
            } else {
                print!("{data:02X} ");
            }
        }
    }
}
