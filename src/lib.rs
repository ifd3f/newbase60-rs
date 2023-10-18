#[cfg(tests)]
mod tests;

/// Convert a number into its New Base 60 representation.
///
/// For more information, see [this link](http://tantek.pbworks.com/w/page/19402946/NewBase60).
///
/// # Examples
///
/// Basic usage:
/// ```
/// use newbase60::num_to_sxg;
///
/// assert_eq!(num_to_sxg(1337), "NH".to_string());
/// ```
pub fn num_to_sxg(n: u128) -> String {
    // A static lookup table
    static DIGITS: &[u8; 60] = b"0123456789ABCDEFGHJKLMNPQRSTUVWXYZ_abcdefghijkmnopqrstuvwxyz";

    if n == 0 {
        return "0".to_string();
    }

    let mut n = n;
    let mut s = String::new();
    while n > 0 {
        // Safe to convert because it is mod 60
        let d = n % 60;

        // Safe to index because 0 <= n < 60
        // Safe to convert to char because the lookup table is all chars
        let ch = DIGITS[d as usize] as char;

        s.push(ch);
        n = (n - d) / 60;
    }
    s.chars().rev().collect()
}

/// Convert a string into its New Base 60 representation, dropping invalid characters.
///
/// Valid New Base 60 characters are alphanumeric or underscores (that is, they
/// individually match the regex `[a-zA-Z0-9_]`). Invalid characters will be treated as if
/// they did not exist. Empty strings will evaluate to 0.
///
/// If the resulting value is larger than 2<sup>128</sup>, then this function will return `None`.
///
/// For more information, see [this link](http://tantek.pbworks.com/w/page/19402946/NewBase60).
///
/// # Examples
///
/// ```
/// use newbase60::sxg_to_num;
///
/// assert_eq!(sxg_to_num("NH"), Some(1337));
/// assert_eq!(sxg_to_num("N🥺H"), Some(1337));
/// assert_eq!(sxg_to_num("verylongstringthatoverflowsthemultiplicationbuffer"), None);
/// ```
pub fn sxg_to_num(s: &str) -> Option<u128> {
    let mut n: u128 = 0;
    for c in s.chars() {
        let digit = match c {
            '0'..='9' => c as u8 - b'0',
            'A'..='H' => c as u8 - b'A' + 10,
            'J'..='N' => c as u8 - b'J' + 18,
            'P'..='Z' => c as u8 - b'P' + 23,
            '_' => 34,
            'a'..='k' => c as u8 - b'a' + 35,
            'm'..='z' => c as u8 - b'm' + 46,
            'I' | 'l' => 1, // typo capital I, lowercase l to 1
            'O' => 0,       // error correct typo capital O to 0
            _ => continue,  // skip invalid chars
        };

        n = n.checked_mul(60)?.checked_add(digit as u128)?;
    }
    Some(n)
}
