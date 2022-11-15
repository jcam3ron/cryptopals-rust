mod base64;
use crate::base64::to_base64;

fn main() {

    let str = to_base64("Hello, world!".as_bytes());
    println!("{str}");
}
