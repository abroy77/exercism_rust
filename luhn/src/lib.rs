/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let trimmed_code = code.replace(" ", ""); // remove spaces
    if trimmed_code.len() <= 1 || trimmed_code.chars().any(|x| !x.is_ascii_digit()) {
        return false; // false if invalid chars or length <= 1
    }

    let digits: Vec<u8> = trimmed_code
        .chars()
        .filter_map(|x: char| x.to_digit(10)) // convert to digits
        // filter_map extracts the digits from Some(digit)
        .map(|x| x as u8)
        .rev() // for the checksum we need to start from the right. so we reverse
        .collect();

    let mut sum = 0;
    for i in 0..digits.len() {
        if i % 2 != 0 {
            // every second digit
            sum += if digits[i] >= 5 {
                // if digit*2>9, subtract 9
                digits[i] * 2 - 9
            } else {
                digits[i] * 2
            };
        } else {
            // non-second digits
            sum += digits[i];
        }
    }

    sum % 10 == 0 // check if divisible by 10
}
