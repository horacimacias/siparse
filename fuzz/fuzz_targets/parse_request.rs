#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut headers = [siparse::EMPTY_HEADER; 16];
    let mut req = siparse::Request::new(&mut headers);
    req.parse(data);
});
