/*!
A simple ***.env*** file loader

# Description
Giving a sequence of env files from most general to most specific.

# Operation
Parse each file for key val remove any comments blank lines and extra whitespace.

# Syntax
```no_run
TEST_DATA=bar       # spaces are optional
## this is a comment
TEST_baz = "baz"    # double quotes are removed
## above line was left intentionally blank
```
will produce:

|Key|Value|
|---|---|
`TEST_DATA`|`bar`
`TEST_baz`|`baz`
*/

/// Tries to load the env. vars from these paths
///
/// ```rust
/// // this will add envs it finds from the first to the last
/// // so important (read: secret/user) ends should be at the end of the iterator
/// simple_env_load::load_env_from(&["./env", "~/.config/.env"]);
/// ```
pub fn load_env_from<I, T>(paths: I)
where
    I: IntoIterator<Item = T>,
    T: AsRef<std::path::Path>,
{
    paths
        .into_iter()
        .map(std::fs::read_to_string)
        .flatten()
        .for_each(|data| parse_and_set(&data, |k, v| std::env::set_var(&k, &v)))
}

/// Parse an env string and calls a function for each key=value pair
///
/// This is useful for mocking and testing
///
/// ```rust
/// let data = r#"
/// TEST_DATA=bar # spaces are optional
/// # this is a comment
/// TEST_baz = "baz" # double quote are removed
///
/// # above line was left intentionally blank
/// "#;
/// # assert!(std::env::var("TEST_DATA").is_err());
/// # assert!(std::env::var("TEST_baz").is_err());
/// // just set the env. vars, but this can be any fn(&str, &str)
/// simple_env_load::parse_and_set(&data, |k, v| std::env::set_var(k, v));
/// assert_eq!(std::env::var("TEST_DATA").unwrap(), "bar");
/// assert_eq!(std::env::var("TEST_baz").unwrap(), "baz");
/// ```
pub fn parse_and_set(data: &str, set: fn(k: &str, v: &str)) {
    data.lines()
        .filter_map(|s| Some(s.trim()).filter(|s| !s.starts_with('#')))
        .map(|s| {
            s.splitn(2, '=').filter_map(|mut s| {
                if let Some(right) = s.chars().position(|c| c == '#') {
                    s = &s[..right]
                }
                Some(s.trim()).filter(|s| !s.is_empty())
            })
        })
        .flat_map(|mut iter| Some((iter.next()?, iter.next()?.replace('"', ""))))
        .for_each(|(k, v)| set(k, &v))
}
