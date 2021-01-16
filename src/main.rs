use std::{fs, fmt};
use serde::{Deserialize, Deserializer, de};
use toml;

#[derive(Debug, Deserialize)]
struct TestStruct {
    #[serde(deserialize_with ="callback")]
    optional: Option<i32>,
    notoptional: i32,
}

struct TestVisitor {}

impl<'de> de::Visitor<'de> for TestVisitor {
    type Value = Option<i32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "")
    }
}


fn callback<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<i32>, D::Error> {
    println!("Not called!!!");
    let visitor = TestVisitor {};
    deserializer.deserialize_option(visitor)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test: TestStruct = toml::from_str(&fs::read_to_string("test.toml")?)?;
    println!("{:?}", test);
    Ok(())
}
