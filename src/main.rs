extern crate text2image;

fn main() {
    let data = text2image::
    let img = text2image::text2image(String::from("ほげほげ"));
    img.save("hoge.png").unwrap();
}
