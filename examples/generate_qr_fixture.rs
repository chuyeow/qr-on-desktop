use image::Luma;
use qr_on_desktop::fixtures::load_cases;
use qrcode::QrCode;
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
