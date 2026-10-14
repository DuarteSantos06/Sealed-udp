use std::collections::HashSet;

use sealed_udp::crypto::{random_connection_id, random_nonce};
use sealed_udp::protocol::{CONNECTION_ID_SIZE, NONCE_SIZE};

#[test]
fn nonce_has_expected_size() {
    assert_eq!(random_nonce().len(), NONCE_SIZE);
}

#[test]
fn nonce_is_not_all_zero() {
    assert_ne!(random_nonce(), [0u8; NONCE_SIZE]);
}

#[test]
fn nonces_do_not_repeat() {
    let nonces: HashSet<_> = (0..1000).map(|_| random_nonce()).collect();
    assert_eq!(nonces.len(), 1000);
}

#[test]
fn connection_id_has_expected_size() {
    assert_eq!(random_connection_id().len(), CONNECTION_ID_SIZE);
}

#[test]
fn connection_ids_do_not_repeat() {
    // 64-bit ids: a collision in 1000 draws is astronomically unlikely.
    let ids: HashSet<_> = (0..1000).map(|_| random_connection_id()).collect();
    assert_eq!(ids.len(), 1000);
}

#[test]
fn nonce_bytes_are_spread() {
    // Over many nonces every byte position should take more than one value.
    let nonces: Vec<_> = (0..64).map(|_| random_nonce()).collect();
    for pos in 0..NONCE_SIZE {
        let values: HashSet<u8> = nonces.iter().map(|n| n[pos]).collect();
        assert!(values.len() > 1, "byte {pos} never changed");
    }
}
