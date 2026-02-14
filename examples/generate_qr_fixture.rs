use image::Luma;
use qrcode::QrCode;
use qr_on_desktop::fixtures::load_cases;
use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cases = load_cases()?;
    let output_dir = Path::new("tests/fixtures");
    fs::create_dir_all(output_dir)?;

    for case in cases {
        let code = QrCode::new(case.payload.as_bytes())?;
        let image = code.render::<Luma<u8>>().build();
        let output_path = output_dir.join(format!("{}.png", case.name));
        image.save(&output_path)?;
        println!("Wrote {output_path:?}");
        println!("Payload: {}", case.payload);
        println!("----");
    }

    Ok(())
}

type Cases = Vec<(String, String)>;

fn load_cases() -> Result<Cases, Box<dyn std::error::Error>> {
    let raw = include_str!("../tests/fixtures/cases.tsv");
    let mut cases = Vec::new();

    for (line_no, line) in raw.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (name, raw_payload) = line
            .split_once('\t')
            .ok_or_else(|| format!("invalid case format at line {}", line_no + 1))?;

        let payload = raw_payload.replace("\\n", "\n");
        cases.push((name.to_string(), payload));
    }

    if cases.is_empty() {
        return Err("no cases found".into());
    }

    Ok(cases)
}
