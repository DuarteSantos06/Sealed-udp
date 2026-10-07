use rand_core::{OsRng, RngCore};
use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::protocol::{CONNECTION_ID_SIZE, NONCE_SIZE, PUBLIC_KEY_SIZE};

pub fn generate_keypair() -> (EphemeralSecret, PublicKey) {
    let secret = EphemeralSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    (secret, public)
}

pub fn random_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn random_connection_id() -> [u8; CONNECTION_ID_SIZE] {
    let mut id = [0u8; CONNECTION_ID_SIZE];
    OsRng.fill_bytes(&mut id);
    id
}

pub fn compute_shared(secret: EphemeralSecret, peer_public: [u8; PUBLIC_KEY_SIZE]) -> [u8; 32] {
    let peer = PublicKey::from(peer_public);
    secret.diffie_hellman(&peer).to_bytes()
}