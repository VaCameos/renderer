use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1, 1);
}

#[wasm_bindgen_test]
fn fail() {
    assert_eq!(1, 2);
}

// On a target other then `wasm32-unknown-unknown`, the `#[test]` attribute
// will be used instead, allowing this test to run on any target.
#[wasm_bindgen_test(unsupported = test)]
fn all_targets() {
    assert_eq!(1, 2);
}