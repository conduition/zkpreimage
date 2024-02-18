#![no_main]
#![no_std]

use k256::elliptic_curve::group::GroupEncoding as _;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha;
use sha::Sha256 as _;

risc0_zkvm::guest::entry!(main);

#[allow(dead_code)]
fn secp256k1_ec_mul(secret: &[u8; 32]) -> [u8; 33] {
    let seckey = k256::SecretKey::from_bytes(secret.into()).expect("secret key is invalid");
    let pubkey = seckey.public_key();
    pubkey.as_affine().to_bytes().into()
}

fn main() {
    let input: [u8; 32] = env::read();
    let mut output = [0u8; 32 + 33];

    // Compute SHA256 hash, and write it to the journal as a public output
    let digest = sha::Impl::hash_bytes(&input);
    output[..32].copy_from_slice(digest.as_bytes());

    // Compute the public key of the given input.
    // Uncommenting these lines out will cause the host to panic.
    let point = secp256k1_ec_mul(&input);
    output[32..].copy_from_slice(&point);

    let output_bytes: &[u8] = output.as_ref();
    env::commit(&output_bytes);
}
