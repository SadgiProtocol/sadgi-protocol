#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // 1. Read the input (e.g., a simple greeting request)
    let input: u32 = env::read();

    // 2. Perform some simple operation to prove execution
    let output = input * 2;

    // 3. Commit the public output to the Journal
    env::commit(&output);
}
