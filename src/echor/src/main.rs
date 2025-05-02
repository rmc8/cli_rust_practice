use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT") // 引数の値を表示する際の名称
                .help("Input text") // ヘルプメッセージ
                .required(true) // この引数は必須
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("\n")
                .help("Do not print newline")
                .takes_value(false), // ヘルプメッセージ
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    let mut ending = "\n";
    if omit_newline {
        ending = "";
    }
    print!("{}{}", text.join(" "), ending);
}
