#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Hello World Logic
    let is_hello = true;
    sp1_zkvm::io::commit(&is_hello);
}
