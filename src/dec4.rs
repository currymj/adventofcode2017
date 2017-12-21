use std::collections::HashSet;
use std::iter::FromIterator;

pub fn passphrase(x: &str) -> bool {
    let xvec: Vec<&str> = x.split(" ").collect();
    let xset: HashSet<&str> = HashSet::from_iter(xvec.iter().cloned());
    xset.len() == xvec.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dec4test() {
        assert!(passphrase("aa bb cc dd ee"));
        assert!(!passphrase("aa bb aa dd ee"));
    }
}