#[cfg(feature = "nightly")]
#[test]
fn test_array() {
    use bitwrap::BitWrap;

    #[derive(Default, BitWrap)]
    struct Packet {
        #[bits]
        data: [u8; 4],
    }

    const DATA: &[u8] = &[0xF0, 0x9F, 0xA6, 0x80];

    let mut packet = Packet::default();
    let result = packet.unpack(DATA).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(packet.data, DATA);

    let mut buffer: [u8; 4] = [0; 4];
    let result = packet.pack(&mut buffer).unwrap();

    assert_eq!(result, DATA.len());
    assert_eq!(buffer, DATA);
}