use qr_on_desktop::{decode_file, fixtures::load_cases};
use std::path::PathBuf;

#[test]
fn it_decodes_all_fixture_cases() {
    let cases = load_cases().expect("fixture case list should be valid");
    for case in cases {
        let image_path = PathBuf::from("tests/fixtures").join(format!("{}.png", case.name));
        let values = decode_file(&image_path).unwrap_or_else(|err| {
            panic!("failed to decode fixture {}: {err}", image_path.display())
        });

        assert!(
            values.iter().any(|value| value == &case.payload),
            "fixture {} expected payload not found in {values:?}",
            case.name
        );
    }
}

#[test]
fn it_decodes_legacy_sample_fixture() {
    let values = decode_file(PathBuf::from("tests/fixtures/qr-on-desktop-sample.png").as_path())
        .expect("failed to decode legacy fixture");
    assert!(
        values.iter().any(|value| value == "https://example.com/qr-on-desktop"),
        "legacy fixture did not include expected payload: {values:?}"
    );
}
