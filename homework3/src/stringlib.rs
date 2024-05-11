use slug::slugify;

pub fn to_lowercase(input_string: String) -> String {
    input_string.to_lowercase()
}
pub fn to_uppercase(input_string: String) -> String {
    input_string.to_uppercase()
}
pub fn to_no_spaces(input_string: String) -> String {
    input_string.trim().replace(' ', "-").to_lowercase()
}
pub fn to_snakecase(input_string: String) -> String {
    input_string.trim().replace(' ', "-").to_lowercase()
}
pub fn to_slugified(input_string: String) -> String {
    slugify(input_string)
}
