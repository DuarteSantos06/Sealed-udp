use sealed_udp::crypto::{encrypt, random_nonce};
use sealed_udp::protocol::PUBLIC_KEY_SIZE;

// `encrypt` is still a stub; these tests pin its current contract so a real
// implementation shows up as a deliberate change.

#[test]
fn encrypt_does_not_panic_on_empty_buffer() {
    encrypt(&Vec::new(), random_nonce(), [0u8; PUBLIC_KEY_SIZE]);
}

#[test]
fn encrypt_leaves_input_untouched() {
    let buf = b"plaintext".to_vec();
    let before = buf.clone();
    encrypt(&buf, random_nonce(), [1u8; PUBLIC_KEY_SIZE]);
    assert_eq!(buf, before);
}
