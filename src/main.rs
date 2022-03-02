use curve25519_dalek::ristretto::RistrettoPoint;
use sha2::Sha512;

use mc_account_keys::PublicAddress;
use mc_api::printable::PrintableWrapper;
use mc_crypto_keys::RistrettoPublic;

// Start with a fixed string, that could not possibly be a chosen plaintext.
const BURN_ADDRESS: &str = "burn address";

fn main() {
    // Hash the known string to create a Ristretto point, which makes it part of a valid public key.
    let pt = RistrettoPoint::hash_from_bytes::<Sha512>(BURN_ADDRESS.as_bytes());

    // Construct a public address, using the hashed point as both the view and spend key. This
    // bypasses the normal way of constructing addresses using private keys and subaddress indices.
    let address = PublicAddress::new(&RistrettoPublic::from(pt), &RistrettoPublic::from(pt));

    // Output the address in the standard format, using base-58 encoding.
    let mut wrapper = PrintableWrapper::new();
    wrapper.set_public_address((&address).into());
    let b58_address = wrapper.b58_encode().unwrap();
    println!("{}", b58_address);
}
