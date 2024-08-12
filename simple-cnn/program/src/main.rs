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
    let fc_weights_a = sp1_zkvm::io::read_vec();
    let fc_weights_b = sp1_zkvm::io::read_vec();

    let signal_len = signal.len();
    let fc_weights_a_len = fc_weights_a.len();
    let fc_weights_b_len = fc_weights_b.len();

    // Define the first fully-connected layer
    assert_eq!(fc_weights_a_len % signal_len, 0);
    let fc_weights_a_num_rows = fc_weights_a_len / signal_len;

    let mut conv_output = Vec::<u32>::new();
    for i in 0..fc_weights_a_num_rows {
        let mut row_sum = 0u32;
        for j in 0..signal_len {
            row_sum += (fc_weights_a[i * signal_len + j] as u32) * (signal[j] as u32);
        }
        conv_output.push(row_sum);
    }

    // Define the second fully-connected layer
    assert_eq!(fc_weights_b_len % conv_output.len(), 0);
    let fc_weights_b_num_rows = fc_weights_b_len / conv_output.len();

    let mut output = Vec::<u32>::new();
    for i in 0..fc_weights_b_num_rows {
        let mut row_sum = 0u32;
        for j in 0..conv_output.len() {
            row_sum += (fc_weights_b[i * conv_output.len() + j] as u32) * (conv_output[j] as u32);
        }
        output.push(row_sum);
    }

    // Encocde the public values of the program.
    let bytes = PublicValuesTuple::abi_encode(&(signal_len as u32, 0));

    // Commit to the public values of the program.
    sp1_zkvm::io::commit_slice(&bytes);
}
