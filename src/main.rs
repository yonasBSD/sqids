use sqids::{Sqids, Options};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
/*
    let sqids = Sqids::default();
    let id = sqids.encode(&[1, 2, 3])?; // "86Rf07"
    let numbers = sqids.decode(&id); // [1, 2, 3]

    print!("{:#?}", id);
    print!("{:#?}", numbers);
*/

    // Randomize IDs by providing a custom alphabet:
    let sqids = Sqids::new(Some(Options::new(
      Some("FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE".to_string()),
      None,
      None,
    )))?;

    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    let numbers: Vec<u8> = numbers.as_bytes().to_vec();
    let numbers: Vec<u64> = numbers.iter()
        .map(|i|
          u64::from(*i)
        )
        .collect();

    let id = sqids.encode(numbers.as_slice())?;

    println!("encoded: {:#?}", id);

    let numbers: Vec<u8> = sqids.decode(&id).iter()
      .map(|i|
        *i as u8
      )
      .collect();

    println!("decoded: {}", std::str::from_utf8(&numbers).unwrap());

    Ok(())
}
