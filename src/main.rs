use curve25519_dalek::ristretto::RistrettoPoint;
use sha2::Sha512;

use mc_account_keys::PublicAddress;
use mc_api::printable::PrintableWrapper;
use mc_crypto_keys::RistrettoPublic;

const BURN_ADDRESS: &str = "burn address";

fn main() {
    let pt = RistrettoPoint::hash_from_bytes::<Sha512>(BURN_ADDRESS.as_bytes());
    let address = PublicAddress::new(&RistrettoPublic::from(pt), &RistrettoPublic::from(pt));
    let mut wrapper = PrintableWrapper::new();
    wrapper.set_public_address((&address).into());
    let b58_address = wrapper.b58_encode().unwrap();
    println!("{}", b58_address);
}
