use image::GrayImage;
use rqrr::PreparedImage;

pub fn decode_qr_from_gray_image(image: GrayImage) -> Vec<String> {
    let mut prepared = PreparedImage::prepare(image);
    let mut values = Vec::new();
    for grid in prepared.detect_grids() {
        if let Ok((_meta, value)) = grid.decode() {
            values.push(value);
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::decode_qr_from_gray_image;
    use image::Luma;

    #[test]
    fn decodes_payload_from_generated_qr() {
        let payload = "https://example.com/qr-on-desktop";
        let code = qrcode::QrCode::new(payload.as_bytes()).unwrap();
        let image = code.render::<Luma<u8>>().build();

        let decoded = decode_qr_from_gray_image(image);
        assert!(decoded.contains(&payload.to_string()));
    }

    #[test]
    fn returns_empty_for_blank_image() {
        let image = image::GrayImage::from_pixel(32, 32, Luma([255]));
        assert!(decode_qr_from_gray_image(image).is_empty());
    }
}
