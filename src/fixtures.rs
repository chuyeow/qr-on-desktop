use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct FixtureCase {
    pub name: String,
    pub payload: String,
}

pub fn load_cases() -> Result<Vec<FixtureCase>> {
    let raw = include_str!("../tests/fixtures/cases.tsv");
    let mut cases = Vec::new();

    for (line_no, line) in raw.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (name, payload) = line
            .split_once('\t')
            .with_context(|| format!("invalid case format at line {}", line_no + 1))?;
        cases.push(FixtureCase {
            name: name.to_string(),
            payload: payload.replace("\\n", "\n"),
        });
    }

    if cases.is_empty() {
        anyhow::bail!("fixture case list is empty");
    }

    Ok(cases)
}

#[cfg(test)]
mod tests {
    use super::load_cases;

    #[test]
    fn loads_cases_with_payloads() {
        let cases = load_cases().expect("fixture file should be parseable");
        assert!(!cases.is_empty());
        assert!(cases.iter().any(|case| !case.name.is_empty() && !case.payload.is_empty()));
    }
}
