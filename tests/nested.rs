use {
    std::net::Ipv4Addr,
    bitwrap_extra::*,
};


#[test]
fn test_readme_nested() {
    #[derive(BitWrap)]
    struct IP4 {
        #[bitfield(8)] ttl: u8,
        #[bitfield(8)] protocol: u8,
        #[bitfield(16)] checksum: u16,
        #[bitfield(32)] src: Ipv4Addr,
        #[bitfield(32)] dst: Ipv4Addr,
    }

    const DATA: &[u8] = &[
        0x40, 0x88, 0x37, 0x5D, 0xC0, 0xA8, 0xC8, 0xB0,
        0xC0, 0xA8, 0xC8, 0xB7,
    ];

    let mut packet = IP4 {
        ttl: 0,
        protocol: 0,
        checksum: 0,
        src: Ipv4Addr::new(0, 0, 0, 0),
        dst: Ipv4Addr::new(0, 0, 0, 0),
    };

    packet.unpack(DATA).unwrap();

    assert_eq!(packet.ttl, 64);
    assert_eq!(packet.protocol, 136);
    assert_eq!(packet.checksum, 0x375D);
    assert_eq!(packet.src, Ipv4Addr::new(192, 168, 200, 176));
    assert_eq!(packet.dst, Ipv4Addr::new(192, 168, 200, 183));

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(32, 0);
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(&buffer[.. result], DATA);
}
