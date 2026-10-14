use sealed_udp::crypto::{compute_shared, generate_keypair};
use sealed_udp::protocol::PUBLIC_KEY_SIZE;
use x25519_dalek::PublicKey;

#[test]
fn public_key_matches_secret() {
    let (secret, public) = generate_keypair();
    assert_eq!(PublicKey::from(&secret).to_bytes(), public.to_bytes());
}

#[test]
fn public_key_has_expected_size() {
    let (_, public) = generate_keypair();
    assert_eq!(public.to_bytes().len(), PUBLIC_KEY_SIZE);
}

#[test]
fn keypairs_are_unique() {
    let (_, a) = generate_keypair();
    let (_, b) = generate_keypair();
    assert_ne!(a.to_bytes(), b.to_bytes());
}

#[test]
fn both_sides_derive_same_shared_secret() {
    let (client_secret, client_public) = generate_keypair();
    let (server_secret, server_public) = generate_keypair();

    let client_shared = compute_shared(client_secret, server_public.to_bytes());
    let server_shared = compute_shared(server_secret, client_public.to_bytes());

    assert_eq!(client_shared, server_shared);
}

#[test]
fn shared_secret_is_not_all_zero() {
    let (a_secret, _) = generate_keypair();
    let (_, b_public) = generate_keypair();
    assert_ne!(compute_shared(a_secret, b_public.to_bytes()), [0u8; 32]);
}

#[test]
fn different_peers_give_different_shared_secrets() {
    let (a_secret, a_public) = generate_keypair();
    let (b_secret, _) = generate_keypair();
    let (_, c_public) = generate_keypair();

    let ab = compute_shared(b_secret, a_public.to_bytes());
    let ac = compute_shared(a_secret, c_public.to_bytes());

    assert_ne!(ab, ac);
}

#[test]
fn key_exchange_through_handshake_packets() {
    use sealed_udp::crypto::{random_connection_id, random_nonce};
    use sealed_udp::protocol::{Handshake, Packet, VERSION, parse_packet, serialize};

    let (client_secret, client_public) = generate_keypair();
    let (server_secret, server_public) = generate_keypair();
    let connection_id = random_connection_id();

    let hello = serialize(&Packet::ClientHello(Handshake {
        version: VERSION,
        connection_id,
        nonce: random_nonce(),
        public_key: client_public.to_bytes(),
    }));
    let received_client_key = match parse_packet(&hello) {
        Some(Packet::ClientHello(h)) => h.public_key,
        _ => panic!("expected ClientHello"),
    };

    let reply = serialize(&Packet::ServerHello(Handshake {
        version: VERSION,
        connection_id,
        nonce: random_nonce(),
        public_key: server_public.to_bytes(),
    }));
    let received_server_key = match parse_packet(&reply) {
        Some(Packet::ServerHello(h)) => h.public_key,
        _ => panic!("expected ServerHello"),
    };

    assert_eq!(
        compute_shared(client_secret, received_server_key),
        compute_shared(server_secret, received_client_key)
    );
}
