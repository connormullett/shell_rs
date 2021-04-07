use nom::{
    bytes::streaming::tag,
    character::complete::{alphanumeric1, newline},
    error::{context, VerboseError},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

use std::collections::HashMap;

pub struct ConfigItem {
    pub key: String,
    pub value: String,
}

impl ConfigItem {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn parse_config_pair(input: &str) -> Res<&str, ConfigItem> {
    context(
        "parse_config_pair",
        separated_pair(alphanumeric1, tag("="), alphanumeric1),
    )(input)
    .map(|(next_input, output)| {
        (
            next_input,
            ConfigItem::new(output.0.to_string(), output.1.to_string()),
        )
    })
}

pub fn parse_config(input: &str) -> Res<&str, HashMap<String, String>> {
    context("parse_config", separated_list0(newline, parse_config_pair))(input)
        .map(|(next_input, items)| (next_input, map_vec_to_hash_map(items)))
}

fn map_vec_to_hash_map(items: Vec<ConfigItem>) -> HashMap<String, String> {
    items.iter().map(|item| (item.key, item.value)).collect()
}
