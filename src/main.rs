extern crate reing_text2image;
extern crate clap;

use clap::{App, Arg};
use reing_text2image::TextImage;
use std::path::Path;

fn main() {
    let app = App::new("text2image")
        .arg(Arg::with_name("text")               // 位置引数を定義
            .value_name("TEXT")
            .help("Text to write.")     // ヘルプメッセージ
            .required(true)                         // この引数は必須であることを定義
        )
        .arg(Arg::with_name("brand")
            .value_name("BRAND")
            .help("Brand text. Default: Reing")
            .takes_value(true)
            .long("brand")
            .required(false)
        )
        .arg(Arg::with_name("rgb")              // オプションを定義
            .value_name("RGB")
            .help("Background color. Default: 00,A2,9A")              // ヘルプメッセージ
            .takes_value(true)
            .long("rgb")
            .required(false)                  // 値を持つことを定義
        ).arg(Arg::with_name("output")
            .value_name("path")
            .help("Output file path. Default: ./output.jpg")
            .long("output")
            .takes_value(true)
            .required(false)
        );
    
    let matches = app.get_matches();
    let text = matches.value_of("text").expect("Error: text must be specified.");
    let brand = matches.value_of("brand").unwrap_or("Reing");
    let path = matches.value_of("output").unwrap_or("./output.jpg");
    let color = if let Some(rgb_str) = matches.value_of("rgb") {
        let rgb = rgb_str.split(',').map(|s| u8::from_str_radix(s, 16).expect("Error: each color value should be between 00 and FF.")).collect::<Vec<u8>>();
        (rgb[0], rgb[1], rgb[2])
    } else {
        (0x00,0xA2,0x9A)
    };
    let text_image = TextImage::new(String::from(text), String::from(brand), color);
    text_image.save_image(&Path::new(path)).unwrap();
}
