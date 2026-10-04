pub fn sanitize_folder_name(name: &str) -> String {
    let illegal_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

    let sanitized: String = name
        .chars()
        .filter(|character| !illegal_chars.contains(character) && !character.is_control())
        .collect();

    let trimmed = sanitized.trim().trim_matches('.');

    if trimmed.is_empty() {
        return "Instance".to_string();
    } else {
        return trimmed.to_string();
    }
}
