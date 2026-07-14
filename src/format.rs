//! Port of src/utils/format.js

const ROMAN: [&str; 12] = [
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII",
];

/// `"2026.03"` -> `"III.2026"`
pub fn roman_date(s: &str) -> String {
    let mut parts = s.split('.');
    let year = parts.next().unwrap_or_default();
    let month: usize = parts.next().and_then(|m| m.parse().ok()).unwrap_or(0);
    match ROMAN.get(month.wrapping_sub(1)) {
        Some(r) => format!("{r}.{year}"),
        None => format!("undefined.{year}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_js_roman_date() {
        assert_eq!(roman_date("2026.03"), "III.2026");
        assert_eq!(roman_date("2026.02"), "II.2026");
        assert_eq!(roman_date("2025.11"), "XI.2025");
        assert_eq!(roman_date("2025.12"), "XII.2025");
        assert_eq!(roman_date("2025.01"), "I.2025");
    }
}
