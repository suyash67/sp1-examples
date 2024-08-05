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
    let signal = sp1_zkvm::io::read_vec();
    let kernel = sp1_zkvm::io::read_vec();
    let fc_weights = sp1_zkvm::io::read_vec();
    let n = signal.len() as u32;
    let n_fc = fc_weights.len() as u32;

    if n > 33_020 {
        panic!("This conv-1d program doesn't support n > 33,020, as the result would overflow 32 bits.");
    }

    let signal_len = signal.len();
    let kernel_len = kernel.len();
    let conv_len = signal_len + kernel_len - 1;

    if (conv_len as u32) != n_fc {
        panic!("The output dimension of the convolution must match the fully-connected layer dimension.");
    }

    let mut result = Vec::<u32>::new();
    for _ in 0..conv_len {
        result.push(0);
    }

    for i in 0..signal_len {
        for j in 0..kernel_len {
            result[i + j] += (signal[i] as u32) * (kernel[j] as u32);
        }
    }

    let mut sum = 0u32;
    for i in 0..conv_len {
        sum += result[i] * (fc_weights[i] as u32);
    }

    // Encocde the public values of the program.
    let bytes = PublicValuesTuple::abi_encode(&(n, sum));

    // Commit to the public values of the program.
    sp1_zkvm::io::commit_slice(&bytes);
}
