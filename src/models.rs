use std::num::ParseIntError;

use anyhow::{anyhow, Result};
use serde::{de::Error, Deserialize, Deserializer};

#[derive(PartialEq, Debug)]
pub(crate) struct InitPoint {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[derive(PartialEq, Debug)]
pub(crate) struct WorkingArea {
    pub(crate) width: usize,
    pub(crate) height: usize,
}

fn parse_points(s: &str) -> Result<(usize, usize)> {
    let split: Vec<&str> = s.split('x').take(3).collect(); // Taking 3 instead of 2, to test correctness of the parser

    let unexpected_err = || anyhow!("unexpected error");
    let parse_err = |err: ParseIntError| anyhow!("parsing error: {err}");

    if split.len() == 2 {
        let width = split
            .first()
            .ok_or_else(unexpected_err)
            .and_then(|x| x.parse::<usize>().map_err(parse_err))?;

        let height = split
            .get(1)
            .ok_or_else(unexpected_err)
            .and_then(|x| x.parse::<usize>().map_err(parse_err))?;

        Ok((width, height))
    } else {
        Err(anyhow!(r#"expected format: "1024x768""#))
    }
}

impl<'de> Deserialize<'de> for InitPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        let (x, y) = parse_points(&s)
            .map_err(|err| D::Error::custom(format!("[INIT_POINT ERROR] {err}")))?;

        Ok(InitPoint { x, y })
    }
}

impl<'de> Deserialize<'de> for WorkingArea {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        let (width, height) = parse_points(&s)
            .map_err(|err| D::Error::custom(format!("[WORKING_AREA ERROR] {err}")))?;

        Ok(WorkingArea { width, height })
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::*;

    #[test]
    fn test_serde_correct_points_deserialization() -> Result<()> {
        let example_json = r#"
        {
            "working_area": "150x350",
            "init_point": "200x100"
        }
        "#;

        let test_config: Config = serde_json::from_str(example_json)?;

        // ----------

        assert_eq!(
            test_config.working_area,
            WorkingArea {
                width: 150,
                height: 350
            }
        );

        assert_eq!(test_config.init_point, InitPoint { x: 200, y: 100 });

        Ok(())
    }

    #[test]
    fn test_serde_incorrect_points_deserialization() {
        let example_json = r#"
        {
            "working_area": "150350",
            "init_point": "200x100"
        }
        "#;

        let test_config: Result<Config, _> = serde_json::from_str(example_json);

        assert!(test_config.is_err());
        let err = test_config.unwrap_err();

        assert!(err.is_data());
        assert!(err
            .to_string()
            .starts_with("[WORKING_AREA ERROR] expected format"));

        // ----------

        let example_json = r#"
        {
            "working_area": "whatxwhat",
            "init_point": "200x100"
        }
        "#;

        let test_config: Result<Config, _> = serde_json::from_str(example_json);

        assert!(test_config.is_err());
        let err = test_config.unwrap_err();

        assert!(err.is_data());
        assert!(err
            .to_string()
            .starts_with("[WORKING_AREA ERROR] parsing error"));

        // ----------

        let example_json = r#"
        {
            "working_area": "100x100",
            "init_point": "100100"
        }
        "#;

        let test_config: Result<Config, _> = serde_json::from_str(example_json);

        assert!(test_config.is_err());
        let err = test_config.unwrap_err();

        assert!(err.is_data());
        assert!(err
            .to_string()
            .starts_with("[INIT_POINT ERROR] expected format"));

        // ----------

        let example_json = r#"
        {
            "working_area": "100x100",
            "init_point": "100xwhat"
        }
        "#;

        let test_config: Result<Config, _> = serde_json::from_str(example_json);

        assert!(test_config.is_err());
        let err = test_config.unwrap_err();

        assert!(err.is_data());
        assert!(err
            .to_string()
            .starts_with("[INIT_POINT ERROR] parsing error"));
    }

    #[test]
    fn test_parse_points() -> Result<()> {
        let result = parse_points("1000x1000")?;
        assert_eq!(result.0, 1000);
        assert_eq!(result.1, 1000);

        // ----------

        let result = parse_points("512x100")?;
        assert_eq!(result.0, 512);
        assert_eq!(result.1, 100);

        // ----------

        let result = parse_points("0x0")?;
        assert_eq!(result.0, 0);
        assert_eq!(result.1, 0);

        // ----------

        let result = parse_points("0x0x0");
        assert!(result.is_err());
        let result_err = result.unwrap_err();
        let result_err = result_err.to_string();
        assert!(result_err.contains("expected format"));

        // ----------

        let result = parse_points("0");
        assert!(result.is_err());
        let result_err = result.unwrap_err();
        let result_err = result_err.to_string();
        assert!(result_err.contains("expected format"));

        // ----------

        let result = parse_points("testxtest");
        assert!(result.is_err());
        let result_err = result.unwrap_err();
        let result_err = result_err.to_string();
        assert!(result_err.contains("parsing error"));

        Ok(())
    }
}
