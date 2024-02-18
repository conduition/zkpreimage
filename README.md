# zkpreimage

This repository is a demonstration using [the RISC0 zkVM](https://dev.risczero.com/api/zkvm) to generate a zero-knowledge key-statement proof, which asserts that for some hash $z$ and secp256k1 point $P$, the prover knows a 32-byte secret key $k$ such that:

- $SHA256(k) = z$
- $kG = P$.

This allows the prover to use bitcoin payment tools like Hash-Time Lock Contracts or the [Lightning Network](https://lightning.network) to pay for $k$. In so doing they discover the preimage of $z$, but also can be certain they will learn the discrete log of the point $P$.

This demonstrates in principle the way in which further claims about $k$ could be proven in advance of a preimage sale, such as:

- $k$ is a signature by a valid signature by a given pubkey.
- $k$ is a share in a given Shamir Secret Sharing group.
- $k$ is a valid decryption key for a certain ciphertext.
- $k$ is a solution to a given sudoku puzzle.
- etc.

This opens up a wealth of previously unexplored possibilities to effectively market the sale of secret information in a safe and secure manner.

## UNFINISHED

This repo is currently not fully functional.

## Running the Proof

To run the proof:

```
$ cargo run .
```
