use farmfe_core::resource::Resource;
use imagequant;
use libwebp::WebPDecodeRGBA;
use lodepng::{decode32, encode32, Bitmap, Encoder, RGBA};
use std::collections::HashMap;

pub fn insert_resource(
  resources_map: &mut HashMap<String, Resource>,
  name: String,
  resource: Resource,
) {
  if !resources_map.contains_key(&name) {
    resources_map.insert(name, resource);
  }
}

pub fn compress_png(data: Vec<imagequant::RGBA>, w: usize, h: usize) -> Vec<u8> {
  let mut liq = imagequant::new();
  liq.set_speed(5).unwrap();
  liq.set_quality(10, 99).unwrap();
  let mut img = liq.new_image(data, w, h, 0.0).unwrap();
  // The magic happens in quantize()
  let mut res = match liq.quantize(&mut img) {
    Ok(res) => res,
    Err(err) => panic!("Quantization failed, because: {err:?}"),
  };

  // Enable dithering for subsequent remappings
  res.set_dithering_level(1.0).unwrap();
  let (palette, pixels) = res.remapped(&mut img).unwrap();

  let mut encoder = Encoder::new();
  encoder.set_palette(palette.as_slice()).unwrap();

  let png_vec = encoder.encode(pixels.as_slice(), w, h).unwrap();

  return png_vec;
}

pub fn convert_webp_to_png(webp: &[u8], name: String) -> Vec<u8> {
  let decode_result = WebPDecodeRGBA(webp);
  let result = match decode_result {
    Ok((w, h, webp_box)) => (w, h, webp_box),
    Err(error) => panic!("Problem decoding the webp: {error:?} {name}"),
  };

  let (width, height, buf) = result;
  let png = encode32(&buf, width as usize, height as usize).unwrap();
  let png_rgba = decode32(png).unwrap();

  let png_bytes = compress_png(png_rgba.buffer, width as usize, height as usize);

  return png_bytes;
}

pub fn get_png_bitmap(png_data: &[u8]) -> Bitmap<RGBA> {
  let png_rgba = decode32(png_data).unwrap();
  return png_rgba;
}
