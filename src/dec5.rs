pub fn followjumps(arr: &Vec<i64>) -> i64 {
    let mut i = 0i64;
    let mut scratch = arr.clone();
    let mut step_count = 0;

    while i >= 0 && i < scratch.len() as i64 {
        step_count += 1;
        let curr_offset = scratch[i as usize];
        scratch[i as usize] += 1;
        i += curr_offset;
    }

    step_count
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smalltests() {
        assert_eq!(followjumps(&vec![0,3,0,1,-3]), 5);
    }
}