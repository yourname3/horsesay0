use std::io::Read;

fn get_stdin() -> String {
    let mut buf = String::new();

    // I/O errors? Who cares? 🐴
    let _ = std::io::stdin().read_to_string(&mut buf);

    buf
}

fn format(text: String) -> (String, i32) {
    let mut formatted = String::new();

    let mut cur_line = 0;
    let max_line = 40;
    let mut best_line = 0;

    for char in text.chars() {
        match char {
            '\r' => { continue; }
            ' ' | '\t' => {
                if cur_line >= max_line {
                    formatted.push('\n');
                    cur_line = 0;
                }
                else {
                    if char == '\t' {
                        formatted.push_str("    ");
                        cur_line += 4;
                    }
                    else {
                        formatted.push(' ');
                        cur_line += 1;
                    }
                }
            },
            '\n' => {
                formatted.push(char);
                cur_line = 0;
            }
            _ => {
                formatted.push(char);
                cur_line += 1;
            }
        }

        if cur_line > best_line {
            best_line = cur_line;
        }
    }

    (formatted, best_line)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let src_text = if args.len() < 2 {
        // Read from stdin
        get_stdin()
    }
    else {
        args[1..].join(" ")
    };

    let (formatted, line_len) = format(src_text);
    let center_x = line_len / 2;

    println!("{}", formatted.trim_end());

    for _ in 0..center_x { print!(" "); }
    println!("\\");
    for _ in 0..center_x { print!(" "); }
    println!(" 🐴");
}
