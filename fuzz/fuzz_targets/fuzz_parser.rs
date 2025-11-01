#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Only process valid UTF-8
    if let Ok(input) = std::str::from_utf8(data) {
        let parser = minipg::parser::GrammarParser::new();
        let _ = parser.parse_string(input, "fuzz.g4");
    }
});

