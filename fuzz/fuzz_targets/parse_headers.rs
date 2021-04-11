#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut headers = [siparse::EMPTY_HEADER; 16];
    siparse::parse_headers(data, &mut headers);
});
