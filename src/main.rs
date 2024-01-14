fn collatz_seq_len(mut n: i32) -> u32 {
    let mut len: u32 = 1;

    while n > 1 {
        if n % 2 == 0 {
            n = n / 2
        } else {
            n = 3 * n + 1
        }
        len += 1;
    }

    len
}

#[test]
fn test_collatz_seq_len() {
    assert_eq!(collatz_seq_len(11),15)
}

fn main() {
    println!("Sequence length: {}", collatz_seq_len(11));
}