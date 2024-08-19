pub(crate) fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0{
        if m < n{
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}