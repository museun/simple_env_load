/*!
A simple ***.env*** file loader

# Description
Giving a sequence of env files from most general to most specific.

# Operation
Parse each file for key val remove any comments blank lines and extra whitespace.

# Syntax
```ignore
TEST_DATA=bar       # spaces are optional
## this is a comment
TEST_BAZ = "baz"    # double quotes are removed
TEST_BAR = 'bar'    # single quotes are removed
## above line was left intentionally blank
```
will produce:

|Key|Value|
|---|---|
`TEST_DATA`|`bar`
`TEST_BAZ`|`baz`
`TEST_BAR`|`bar`
*/

/// Tries to load the env. vars from these paths
///
/// This returns a Vec of all of the key=value pairs it set
///
/// ```rust
/// // this will add envs it finds from the first to the last
/// // so important (read: secret/user) ends should be at the end of the iterator
/// simple_env_load::load_env_from(&["./env", "~/.config/.env"]);
/// ```
pub fn load_env_from<I, T>(paths: I) -> Vec<(String, String)>
where
    I: IntoIterator<Item = T>,
    T: AsRef<std::path::Path>,
{
    paths
        .into_iter()
        .map(std::fs::read_to_string) // TODO make this fallible
        .flatten()
        .fold(Vec::new(), |mut entries, data| {
            parse_and_set(&data, |k, v| entries.push((k.to_string(), v.to_string())));
            entries
        })
}

/// Parse an env string and calls a function for each key=value pair
///
/// This is useful for mocking and testing
///
/// ```rust
/// let data = r#"
/// # this is a comment
/// TEST_DATA=bar                 # spaces are optional
/// TEST_BAZ = "baz"              # double quotes are removed
/// TEST_QUX = 'qux'              # single quotes are removed
/// TEST_FOO = "'nested'"         # nested quotes are preserved
/// TEST_BAR = '"nested"'         # nested quotes are preserved
///
/// # above line was left intentionally blank
/// "#;
/// # for key in ["TEST_DATA", "TEST_baz", "TEST_qux","TEST_FOO", "TEST_BAR"] {
/// #   assert!(std::env::var(key).is_err());
/// # }
/// // just set the env. vars, but this can be any fn(&str, &str)
/// simple_env_load::parse_and_set(&data, |k, v| std::env::set_var(k, v));
/// assert_eq!(std::env::var("TEST_DATA").unwrap(), "bar");
/// assert_eq!(std::env::var("TEST_BAZ").unwrap(), "baz");
/// assert_eq!(std::env::var("TEST_QUX").unwrap(), "qux");;
/// assert_eq!(std::env::var("TEST_FOO").unwrap(), "'nested'");
/// assert_eq!(std::env::var("TEST_BAR").unwrap(), "\"nested\"");
/// ```
pub fn parse_and_set(data: &str, mut set: impl FnMut(&str, &str)) {
    parse(data).for_each(|(k, v)| set(k, v))
}

fn parse(data: &str) -> impl Iterator<Item = (&str, &str)> + '_ {
    data.lines().map(<str>::trim).filter_map(|s| {
        if s.starts_with('#') {
            return None;
        }

        let mut iter = s.splitn(2, '=').map(<str>::trim).map(parse_str);
        let (head, tail) = (iter.next()??, iter.next()??);
        Some((head, tail))
    })
}

fn parse_str(input: &str) -> Option<&str> {
    if !input.contains(|c| matches!(c, '"' | '\'')) {
        return input.splitn(2, '#').map(<str>::trim).next();
    }

    #[derive(Debug)]
    enum Flavor {
        Single,
        Double,
        Unknown,
    }

    let mut flavor = Flavor::Unknown;
    let (mut start, mut end) = (None, None);

    for (i, c) in input.char_indices() {
        if start.is_some() && end.is_some() {
            break;
        }

        if matches!(flavor, Flavor::Unknown) {
            flavor = match c {
                '\'' => Flavor::Single,
                '"' => Flavor::Double,
                _ => continue,
            };
        }

        if match flavor {
            Flavor::Single => '\'',
            Flavor::Double => '"',
            Flavor::Unknown => unreachable!(),
        } != c
        {
            continue;
        }

        match (start, end) {
            (None, ..) => {
                start.get_or_insert(i + 1);
            }
            (Some(_), None) => {
                end.get_or_insert(i - 1);
            }
            _ => {}
        };
    }

    let (start, end) = (start?, end?);
    input.get(start..start + end)
}

#[test]
fn parse_octos_in_strings() {
    macro_rules! val {
        ($k:expr => $v:expr) => {
            &[($k, $v)]
        };
    }

    #[rustfmt::skip]
    let tests: &[(&str, &[(&str, &str)])] = &[
        (r##"FOO="#bar""##, val!("FOO"  => "#bar")),
        (r"'asdf'='fdsa'",  val!("asdf" => "fdsa")),
        (r##"#FOO="bar""##, &[]),
    ];
    for (input, expected) in tests {
        assert_eq!(parse(input).collect::<Vec<_>>(), *expected);
    }
}
