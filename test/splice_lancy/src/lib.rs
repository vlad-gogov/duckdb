use splice::{Executable, kernel};
use splice_lancy::Lancy;

#[kernel]
fn add_bias(#[comptime] bias: i32, input: i32) -> i32 {
    input.wrapping_add(bias)
}

#[repr(C)]
pub struct SmokeResult {
    pub status: i32,
    pub value: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn duckdb_splice_lancy_hello(bias: i32, input: i32) -> SmokeResult {
    let result = add_bias::build::<Lancy>(bias).and_then(|kernel| kernel.call(&[input], &mut []));
    match result {
        Ok(value) => SmokeResult { status: 0, value },
        Err(message) => {
            eprintln!("Splice/Lancy: {message}");
            SmokeResult {
                status: 1,
                value: 0,
            }
        }
    }
}
