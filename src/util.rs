#[macro_use]
extern crate stdweb;

use stdweb::unstable::TryInto;

pub fn js_rand(bottom: u8, top: u8) -> u8 {
    let rand = js! { return Math.random(); };
    let base: f64 = rand.try_into().unwrap();
    (base * top as f64).floor() as u8 + bottom
}

pub fn gen_range_avoiding(bottom: u8, top: u8, avoid: Vec<u8>) -> u8 {
    let mut ret = avoid[0];

    while avoid.contains(&ret) {
        ret = js_rand(bottom, top);
    }
    ret
}
