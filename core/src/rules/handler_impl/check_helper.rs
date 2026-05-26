pub fn contains_number_or_hungarian_letter(s: &str) -> bool {
    let hungarian_special_chars = [
        'á', 'é', 'í', 'ó', 'ö', 'ő', 'ú', 'ü', 'ű', 'Á', 'É', 'Í', 'Ó', 'Ö', 'Ő', 'Ú', 'Ü', 'Ű',
    ];

    s.chars()
        .any(|c| c.is_numeric() || hungarian_special_chars.contains(&c))
}
