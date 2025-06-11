pub mod views;
pub mod components;

pub fn to_kebab_case(str: &str) -> String {
	str.to_lowercase()
		.replace(" ", "-")
		.replace(",", "")
		.replace("/", "")
}