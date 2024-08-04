//! A simple program that takes a number `n` as input along with vectors `a` and `b`, and writes the
//! inner `<a, b>` as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{sol, SolType};

/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type PublicValuesTuple = sol! {
    tuple(uint32, uint32)
};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let a_vector = sp1_zkvm::io::read_vec();
    let b_vector = sp1_zkvm::io::read_vec();
    let n = a_vector.len() as u32;

    if n > 330_020 {
        panic!("This inner-product program doesn't support n > 33,020, as the result would overflow 32 bits.");
    }

    if a_vector.len() != b_vector.len() {
        panic!("Lengths of the two vectors a and b do not match.");
    }

    // Compute the inner-product <a, b> fibonacci number, using normal Rust code.
    let mut sum = 0u32;
    for i in 0..n {
        let c: u32 = (a_vector[i as usize] as u32) * (b_vector[i as usize] as u32);
        sum += c;
    }

    // Encocde the public values of the program.
    let bytes = PublicValuesTuple::abi_encode(&(n, sum));

    // Commit to the public values of the program.
    sp1_zkvm::io::commit_slice(&bytes);
}
