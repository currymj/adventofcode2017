fn row_checksum(row: &str) -> Option<i64> {
        let splitrow: Vec<_> = row.split(char::is_whitespace).flat_map(|z| {
            z.parse::<i64>()
        }).collect();
        let max = splitrow.iter().max()?;
        let min = splitrow.iter().min()?;
        Some(max - min)
}
pub fn checksum(x: &str) -> i64 {
    x.split("\n").filter_map(|y| row_checksum(y)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dec2test() {
        assert_eq!(checksum("5 1 9 5\n7 5 3\n2 4 6 8"), 18);
    }
}