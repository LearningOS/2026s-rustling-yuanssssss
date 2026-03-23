// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// 

/// 根据一天中的时间返回冰箱中剩余的冰淇淋数量。
///
/// 如果时间在晚上 10 点之前，冰箱里还有 5 块冰淇淋。到了晚上 10 点，
/// 有人把它们都吃光了，所以就没有剩余了。
///
/// # Arguments
///
/// * `time_of_day` - 使用 24 小时制表示的时间（0-23），其中 0 表示午夜 12 点，
///   22 表示晚上 10 点。
///
/// # Returns
///
/// * `Some(u16)` - 如果时间在 22 点之前，返回 `Some(5)` 表示还有 5 块冰淇淋。
/// * `None` - 如果时间是 22 点或之后，或者 `time_of_day` 大于 23，返回 `None` 表示没有冰淇淋。
///
/// # Examples
///
/// ```
/// // 假设函数已实现
/// assert_eq!(maybe_icecream(21), Some(5));
/// assert_eq!(maybe_icecream(22), None);
/// assert_eq!(maybe_icecream(24), None);
/// ```
// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day <22 {
        Some(5u16)
    } else if time_of_day <24 {
        Some(0u16)
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
