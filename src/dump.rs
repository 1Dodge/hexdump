use crate::Cli;
#[cfg(windows)]
use crate::color::*;
use std::fs;

#[derive(Debug)]
enum AddrMode {
    Default, // no -s, -n, -e args were passed
    S,       // only -s val arg was passed
    SN,      // s val -n val args were passed
    SE,      // s val -e val args were passed
    NE,      // n val -e val args were passed
    N,       // only -n val arg was passed
    E,       // only -e val arg was passed
}

#[derive(Debug, Clone, Copy)]
pub enum VisuMode {
    Default,        // no -v mode arg was passed
    AsciiGraphic,   // -v ascii   arg was passed
    HighlightZeros, // -v zeros   arg was passed
    HighBytes,      // -v high    arg was passed
    ControlChars,   // -v control arg was passed
}

pub struct Dump {
    data: Vec<u8>,
    filesize: u64,
    file_max_index: u64,
    start_addr: u64,
    end_addr: u64,
    num_bytes: u64,
    addr_mode: AddrMode,
    visu_mode: VisuMode,
}

impl Dump {
    pub fn new(path: &String) -> Dump {
        let data = match fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                println_error!("Error: {}", e);
                std::process::exit(1);
            }
        };

        let filesize: u64 = if data.len() != 0 {
            data.len() as u64
        } else {
            println_error!("Error: Cannot dump file with 0 bytes");
            std::process::exit(1);
        };

        let file_max_index: u64 = filesize - 1;

        // enable ansi support on windows (for colorful console output)
        #[cfg(windows)]
        enable_ansi_support();

        Dump {
            data,
            filesize,
            file_max_index,
            start_addr: 0,
            num_bytes: filesize,
            end_addr: file_max_index,
            addr_mode: AddrMode::Default,
            visu_mode: VisuMode::Default,
        }
    }

    pub fn check_args(&mut self, args: &Cli) {
        // Check correct usage of -s, -n, -e
        match (args.start, args.num_bytes, args.end) {
            // only passed -s val
            (Some(s), None, None) => {
                // check if:
                // start_addr <= file_max_index
                // else: throw error
                if s < self.filesize {
                    self.start_addr = s;
                    self.num_bytes = self.filesize - s;
                    self.addr_mode = AddrMode::S;
                } else {
                    println_error!(
                        "Error: start_addr (-s {}) exceeds file_max_index ({})",
                        s,
                        self.file_max_index
                    );
                    std::process::exit(1);
                }
            }
            // only passed -n val
            (None, Some(n), None) => {
                // check if:
                // num_bytes <= filesize
                // num_bytes != 0
                // else: throw according error
                if n <= self.filesize && n != 0 {
                    self.num_bytes = n;
                    self.end_addr = n - 1;
                    self.addr_mode = AddrMode::N;
                } else {
                    if n > self.filesize {
                        println_error!(
                            "Error: num_bytes (-n {}) exceeds filesize ({})",
                            n,
                            self.filesize
                        );
                    } else {
                        println_error!(
                            "Error: num_bytes (-n {}) can't be 0 (can't show 0 bytes)",
                            n
                        );
                    }
                    std::process::exit(1);
                }
            }
            // only passed -e val
            (None, None, Some(e)) => {
                // check if:
                // end_addr <= file_max_index
                // else: throw error
                if e <= self.file_max_index {
                    self.end_addr = e;
                    self.num_bytes = e + 1;
                    self.addr_mode = AddrMode::E;
                } else {
                    println_error!(
                        "Error: end_addr (-e {}) exceeds file_max_index ({})",
                        e,
                        self.file_max_index
                    );
                    std::process::exit(1);
                }
            }
            // -s val -n val args were passed
            (Some(s), Some(n), None) => {
                // check if:
                // start_addr <= file_max_index
                // num_bytes <= than filesize
                // calculated end_addr (start_addr + num_bytes) <= filesize
                // num_byes != 0 (can't show 0 bytes)
                // else: throw according errors
                if s <= self.file_max_index
                    && n <= self.filesize - s
                    && s + n <= self.filesize
                    && n != 0
                {
                    self.start_addr = s;
                    self.num_bytes = n;
                    self.end_addr = s + n - 1;
                    self.addr_mode = AddrMode::SN;
                } else {
                    if s > self.file_max_index {
                        println_error!(
                            "Error: start_addr (-s {}) exceeds file_max_index ({})",
                            s,
                            self.file_max_index
                        );
                        println_info!("Consider: reduce -s to be in file range\n");
                    }
                    if n > self.filesize {
                        println_error!(
                            "Error: num_bytes (-n {}) exceeds filesize ({})",
                            n,
                            self.filesize
                        );
                        println_info!("Consider: reduce -n to be in file range\n");
                    }
                    if s + n > self.filesize && n != 0 {
                        let e = s + (n - 1);
                        println_error!(
                            "Error: calculated end_addr ({}) exceeds file_max_index ({})",
                            e,
                            self.file_max_index
                        );
                        println_info!(
                            "Consider: reduce -n to be in file range or drop -n arg to dump file from -s to eof\n"
                        );
                    }
                    if n == 0 {
                        println_error!(
                            "Error: num_bytes (-n {}) can't be 0 (can't show 0 bytes)\n",
                            n
                        );
                    }

                    std::process::exit(1);
                }
            }
            // -s val -e val args were passed
            (Some(s), None, Some(e)) => {
                // check if:
                // start_addr <= file_max_index
                // end_addr <= file_max_index
                // start_addr <= end_endr
                // else: throw according error
                if s <= self.file_max_index && e <= self.file_max_index && s <= e {
                    self.start_addr = s;
                    self.end_addr = e;
                    self.num_bytes = (e - s) + 1;
                    self.addr_mode = AddrMode::SE;
                } else {
                    if s > self.file_max_index {
                        println_error!(
                            "Error: start_addr (-s {}) exceeds file_max_index ({})",
                            s,
                            self.file_max_index
                        );
                        println_info!("Consider: reduce -s to be in file range\n");
                    }
                    if e > self.file_max_index {
                        println_error!(
                            "Error: end_addr (-e {}) exceeds file_max_index ({})",
                            e,
                            self.file_max_index
                        );
                        println_info!("Consider: reduce -e to be in file range\n");
                    }
                    if s > e {
                        println_error!(
                            "Error: start_addr (-s {}) is bigger than end_addr (-e {})",
                            s,
                            e
                        );
                        println_info!("Consider: reduce -s to be <= -e\n");
                    }

                    std::process::exit(1);
                }
            }
            // -n val -e val args were passed
            (None, Some(n), Some(e)) => {
                // check if:
                // end_addr <= file_max_index
                // num_bytes <= filesize
                // num_bytes != 0
                // calculated start_addr <= file_max_index
                // calculated start_addr >= 0
                // else: throw according error

                // avoid arithmetic_overflow error if n > e
                let mut s: u64 = 0;
                let mut s_valid: bool = false;
                if n <= e + 1 {
                    s = e - (n - 1);
                    s_valid = true;
                }

                if e <= self.file_max_index
                    && n <= self.filesize
                    && n != 0
                    && s <= self.file_max_index
                    && s_valid
                {
                    self.start_addr = s;
                    self.num_bytes = n;
                    self.end_addr = e;
                    self.addr_mode = AddrMode::NE;
                } else {
                    if e > self.file_max_index {
                        println_error!(
                            "Error: end_addr (-e {}) exceeds file_max_index ({})",
                            e,
                            self.file_max_index
                        );
                        println_info!("Consider: reduce -e to be in file range\n");
                    }
                    if n > self.filesize {
                        println_error!(
                            "Error: num_bytes (-n {}) exceeds filesize ({})",
                            n,
                            self.filesize
                        );
                        println_info!("Consider: reduce -n to be in file range\n");
                    }
                    if n == 0 {
                        println_error!(
                            "Error: num_bytes (-n {}) can't be 0 (can't show 0 bytes)\n",
                            n
                        );
                    }
                    if s_valid && s > self.file_max_index {
                        println_error!(
                            "Error: calculated start_addr ({}) exceeds file_max_index ({})",
                            s,
                            self.file_max_index
                        );
                        println_info!("Consider: reduce -e to be in file range\n");
                    }
                    if !s_valid {
                        println_error!(
                            "Error: num_bytes (-n {}) is bigger than end_addr (-e {})",
                            n,
                            e
                        );
                        println_info!("Consider: reduce -n to be <= -e + 1\n");
                    }
                    std::process::exit(1);
                }
            }
            // no -s val, -n val, -e val args were passed (do nothing, use vals from new())
            (None, None, None) => {}
            // ERROR: all -s val, -n val, -e val args were passed (invalid use of args)
            _ => {
                println_error!("Error: invalid use of args (-s, -n, -e)");
                println_info!("----------------------Usage----------------------");
                println_info!("-s:    Prints data from address -s to end of file");
                println_info!("-n:    Prints -n bytes starting from address 0");
                println_info!("-e:    Prints data from address 0 to address -e");
                println_info!("-s -n: Prints -n bytes starting from address -s");
                println_info!("-s -e: Prints data from address -s to address -e");
                println_info!("-n -e: Prints -n bytes back from address -e");
                println_error!("-s -n -e: INVALID COMBINATION");
                std::process::exit(1);
            }
        };

        // handle -v args
        match &args.visualization {
            Some(v) => {
                let arg = v.to_lowercase();
                match arg.as_str() {
                    "ascii" => {
                        self.visu_mode = VisuMode::AsciiGraphic;
                    }
                    "zeros" => {
                        self.visu_mode = VisuMode::HighlightZeros;
                    }
                    "high" => {
                        self.visu_mode = VisuMode::HighBytes;
                    }
                    "control" => {
                        self.visu_mode = VisuMode::ControlChars;
                    }
                    _ => {
                        println_error!("Error: unknown visualization (-v {v} does not exist)");
                        println_info!("-------------------------Usage--------------------------");
                        println_info!("-v ascii:   Highlights ascii printable bytes");
                        println_info!("-v zeros:   Highlights bytes that have the value 0");
                        println_info!("-v high:    Highlights bytes that have a value >= 0x80");
                        println_info!("-v control: Highlights bytes that are control characters");
                        std::process::exit(1);
                    }
                };
            }
            // if no -v arg was passed do nothing
            None => {}
        };
    }

    fn calc_hex_width(max_num: u64) -> usize {
        let digits = if max_num == 0 {
            1
        } else {
            ((64 - max_num.leading_zeros() + 3) / 4) as usize
        };

        digits.max(4)
    }

    fn format_filesize(filesize: u64) -> String {
        // determine human-readable size
        let (value, unit) = if filesize < 1024 {
            (filesize as f64, "bytes")
        } else if filesize < 1024 * 1024 {
            (filesize as f64 / 1024.0, "kB")
        } else if filesize < 1024 * 1024 * 1024 {
            (filesize as f64 / (1024.0 * 1024.0), "MB")
        } else {
            (filesize as f64 / (1024.0 * 1024.0 * 1024.0), "GB")
        };

        let width = Self::calc_hex_width(filesize - 1);

        if unit == "bytes" {
            format!(
                "{filesize} bytes (EOF: {:0width$X})",
                filesize - 1,
                width = width
            )
        } else {
            format!(
                "{value:.1} {unit} ({filesize} bytes, EOF: {:0width$X})",
                filesize - 1,
                width = width
            )
        }
    }

    pub fn print_dump(&self) {
        println!("Filesize: {}", Self::format_filesize(self.filesize));
        let hex_width = Self::calc_hex_width(self.end_addr);
        println!(
            "Dumping {} bytes from {:0width$X} to {:0width$X}",
            self.num_bytes,
            self.start_addr,
            self.end_addr,
            width = hex_width
        );
        println!();
        let left_base_padding = hex_width / 2;
        let right_base_padding = hex_width - left_base_padding;

        // print the dump head
        print_frame_head(left_base_padding, right_base_padding);

        // print the information
        let start_base = self.start_addr / 16;
        let end_base = self.end_addr / 16;
        for line in start_base..=end_base {
            let base = line * 16;
            // print base addr
            print_frame_part("│");
            print_repeated(" ", left_base_padding);
            print_base_addr(base, hex_width);
            print_repeated(" ", right_base_padding);
            print_frame_part("│ ");

            // print data
            for i in 0..16 {
                let addr = base + i;
                if addr < self.start_addr || addr > self.end_addr {
                    print!("   "); // out of range padding
                } else {
                    print_byte(self.data[addr as usize], self.visu_mode);
                }

                if i == 7 {
                    print_frame_part("│ ");
                } else if i != 15 {
                    print!(" ");
                }
            }

            // print ascii
            print_frame_part("│ ");
            for i in 0..16 {
                let addr = base + i;
                if addr < self.start_addr || addr > self.end_addr {
                    print!(" ");
                } else {
                    let byte = self.data[addr as usize];
                    if byte.is_ascii_graphic() || byte == b' ' {
                        print!("{}", byte as char);
                    } else {
                        print_dark(".");
                    }
                }
            }
            print_frame_part(" │");
            println!();
        }
        print_frame_foot(left_base_padding, right_base_padding);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_hex_width_test() {
        assert_eq!(Dump::calc_hex_width(0x0000), 4);
        assert_eq!(Dump::calc_hex_width(0xFFFF), 4);
        assert_eq!(Dump::calc_hex_width(0x1FFFF), 5);
        assert_eq!(Dump::calc_hex_width(0xFFFFFFFF), 8);
    }

    #[test]
    fn format_filesize_test() {
        assert_eq!(Dump::format_filesize(1023), "1023 bytes (EOF: 03FE)");
        assert_eq!(
            Dump::format_filesize(1024),
            "1.0 kB (1024 bytes, EOF: 03FF)"
        );
        assert_eq!(
            Dump::format_filesize(1024 * 1024),
            "1.0 MB (1048576 bytes, EOF: FFFFF)"
        );
        assert_eq!(
            Dump::format_filesize(1024 * 1024 * 1024),
            "1.0 GB (1073741824 bytes, EOF: 3FFFFFFF)"
        );
    }
}
