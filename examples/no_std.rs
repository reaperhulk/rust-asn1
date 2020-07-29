#![no_std]

fn main() {
    let data = b"\x30\x06\x02\x01\x01\x02\x01\x03";

    let result: asn1::ParseResult<_> = asn1::parse(data, |d| {
        d.read_element::<asn1::Sequence>()?
            .parse(|d| Ok((d.read_element::<i64>()?, d.read_element::<i64>()?)))
    });

    // Using libc::printf because println! isn't no_std!
    match result {
        Ok((r, s)) => unsafe { libc::printf(b"r=%ld, s=%ld\n\x00".as_ptr() as *const i8, r, s) },
        Err(_) => unsafe { libc::printf("Error\n\x00".as_ptr() as *const i8) },
    };

    let computed = asn1::write(|w| {
        w.write_element_with_type::<asn1::Sequence>(&|w: &mut asn1::Writer| {
            w.write_element(1i64);
            w.write_element(3i64);
        });
    });
    unsafe {
        libc::printf(
            "Original length: %ld\nComputed length: %ld\n\x00".as_ptr() as *const i8,
            data.len() as i64,
            computed.len() as i64,
        );
    }
}
