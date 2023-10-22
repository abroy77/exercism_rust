pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        // we need this check because 0.ilog10() is undefined
        return true;
    }
    let digits: u32 = num.ilog10() + 1; // this counts the number of digits in num

    let sum = (0..digits).try_fold(0, |acc: u32, i| {
        // try fold returns an Option in this case
        acc.checked_add(((num / 10u32.pow(i)) % 10).pow(digits))
        // checked_add returns None if overflow
    });

    match sum {
        Some(sum) => sum == num,
        None => false, // if there was an overflow error, it's not an Armstrong number
    }
}
