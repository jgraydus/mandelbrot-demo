use web_sys;
use crate::state::{CANVAS_HEIGHT, CANVAS_WIDTH, Rect};

const MAX_ITERATIONS: usize = 1000;

// render the given rectangle of the complex plane into an ImageData
pub fn render(complex_coords: &Rect<f64>) -> web_sys::ImageData {
  // log!("render");
  let canvas_h = CANVAS_HEIGHT as f64;
  let canvas_w = CANVAS_WIDTH as f64;
  let canvas_coords = Rect { left: 0.0, top: 0.0, right: canvas_w, bottom: canvas_h };

  // TODO create data directly rather than through an ImageData?
  let image_data = web_sys::ImageData::new_with_sw(canvas_w as u32, canvas_h as u32).unwrap();
  let mut data = image_data.data();

  render0(canvas_coords, complex_coords, &mut data);

  let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&data),
        canvas_w as u32,
        canvas_h as u32
      ).unwrap();

  image_data
}

// iterate z_(i+1) = z_i^2 + c until z > 2 or the maximum
// number of iterations is reached. return the number of
// iterations. the computation is arranged so that only 3
// multiplications are required in each iteration
fn count_iterations(x0: f64, y0: f64) -> usize {
  let mut x = 0.0;
  let mut y = 0.0; 
  let mut x2 = 0.0;
  let mut y2 = 0.0;
  let mut c = 0;
  while x2+y2 <= 4.0 && c < MAX_ITERATIONS {
    y = (x + x) * y + y0;
    x = x2 - y2 + x0;
    x2 = x * x;
    y2 = y * y;
    c = c + 1;
  }
  c
}

fn render0(
  canvas_coords: Rect<f64>,
  complex_coords: &Rect<f64>,
  data: &mut wasm_bindgen::Clamped<Vec<u8>>
) {
  let canvas_w = canvas_coords.right - canvas_coords.left;
  let canvas_h = canvas_coords.bottom - canvas_coords.top;
  let complex_w = complex_coords.right - complex_coords.left;
  let complex_h = complex_coords.bottom - complex_coords.top;

  for x in 0..(canvas_w as usize) {
    for y in 0..(canvas_h as usize) {
      let x0 = (x as f64 / canvas_w) * complex_w + complex_coords.left;
      let y0 = (y as f64 / canvas_h) * complex_h + complex_coords.top;
      let c = count_iterations(x0, y0);

      // calculate the color for this pixel
      let (r, g, b) = if c < MAX_ITERATIONS {
        let c = c % 32;
        ((c * 5) as u8, (c * 7) as u8, (c * 13) as u8)
      } else {
        (0, 0, 0)
      };

      let idx = (x + y*(canvas_w as usize)) * 4;
      data[idx] = r;
      data[idx+1] = g;
      data[idx+2] = b;
      data[idx+3] = 255; // alpha channel
    }
  }
}

