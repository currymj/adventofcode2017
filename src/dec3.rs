use std::num;

pub fn spiral(num: i64) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut curr_magnitude = 1;
    let mut curr_number = 1;

    'outer: loop {
        // moving right
        while x < curr_magnitude {
            if curr_number == num {
                break 'outer;
            }
            curr_number += 1;
            x += 1;
        }

        // moving up
        while y < curr_magnitude {
            if curr_number == num {
                break 'outer;
            }
            curr_number += 1;
            y += 1;
        }

        // moving left
        while x > -curr_magnitude {
            if curr_number == num {
                break 'outer;
            }
            curr_number += 1;
            x -= 1;
        }

        // moving down
        while y > -curr_magnitude {
            if curr_number == num {
                break 'outer;
            }
            curr_number += 1;
            y -= 1;
        }

        curr_magnitude += 1;
    }


    x.abs() + y.abs()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn dec3test() {
        assert_eq!(spiral(1), 0);
        assert_eq!(spiral(12), 3);
        assert_eq!(spiral(23), 2);
    }
}