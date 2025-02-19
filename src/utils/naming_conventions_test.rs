use crate::utils::name_conventions::NamingConventions;
use rstest::rstest;

#[rstest]
fn from_camel_case() {
    let value = "CamelCase";
    let value = NamingConventions::from(value);
    assert_eq!(
        vec!["camel", "case"],
        value.tokens
    )
}

#[rstest]
fn from_pascal_case() {
    let value = "pascalCase";
    let value = NamingConventions::from(value);
    assert_eq!(
        vec!["pascal", "case"],
        value.tokens
    )
}

#[rstest]
fn from_kebab_case() {
    let value = "kebab-case";
    let value = NamingConventions::from(value);
    assert_eq!(
        vec!["kebab", "case"],
        value.tokens
    )
}

#[rstest]
fn from_snake_case() {
    let value = "snake_case";
    let value = NamingConventions::from(value);
    assert_eq!(
        vec!["snake", "case"],
        value.tokens
    )
}

#[rstest]
fn from_screaming_snake_case() {
    let value = "SNAKE_CASE";
    let value = NamingConventions::from(value);
    assert_eq!(
        vec!["snake", "case"],
        value.tokens
    )
}

#[rstest]
fn to_camel_case() {
    let value = "RustIsAwesome";
    let value = NamingConventions::to_camel_case(value);
    assert_eq!(
        "RustIsAwesome",
        value
    )
}

#[rstest]
fn to_pascal_case() {
    let value = "RustIsAwesome";
    let value = NamingConventions::to_pascal_case(value);
    assert_eq!(
        "rustIsAwesome",
        value
    )
}

#[rstest]
fn to_kebab_case() {
    let value = "RustIsAwesome";
    let value = NamingConventions::to_kebab_case(value);
    assert_eq!(
        "rust-is-awesome",
        value
    )
}

#[rstest]
fn to_snake_case() {
    let value = "RustIsAwesome";
    let value = NamingConventions::to_snake_case(value);
    assert_eq!(
        "rust_is_awesome",
        value
    )
}

#[rstest]
fn to_screaming_snake_case() {
    let value = "RustIsAwesome";
    let value = NamingConventions::to_screaming_snake_case(value);
    assert_eq!(
        "RUST_IS_AWESOME",
        value
    )
}