extern crate itertools;
use self::itertools::zip;

pub fn captcha(x: &str) -> i64 {
    let xvec: Vec<_> = x.split("").flat_map(|z| z.parse::<i64>()).collect();
    let mut yvec: Vec<_> = vec![0; xvec.len()];
    yvec[..xvec.len()-1].clone_from_slice(&xvec[1..]);
    yvec[xvec.len()-1] = xvec[0];

    zip(&xvec, &yvec).filter_map(|(a, b)| {
        if a == b {
            Some(a)
        } else {
            None
        }
    }).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smalltests() {
        assert_eq!(captcha("1122"), 3);
        assert_eq!(captcha("1111"), 4);
    }
}