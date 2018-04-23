extern crate cast;
extern crate minimp3;

use std::mem;

use cast::i32;
use minimp3::safe::Context;
use minimp3::safe::ErrorMessage;

const SQUEAK: &[u8] = include_bytes!("squeak.mp3");

#[test]
fn foo() {
    let mut context = Context::new();
    let mut pcm = [0; Context::MAX_SAMPLES_PER_FRAME];
    let frame = context.decode_frame(SQUEAK, &mut pcm).expect("success");
    assert_eq!(1152, frame.samples);
}
