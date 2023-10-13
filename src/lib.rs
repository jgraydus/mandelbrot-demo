#[macro_use]
mod util;
mod draw;
mod state;

use futures::channel::mpsc::channel;
use futures::stream::StreamExt;
use wasm_bindgen::prelude::*;
use web_sys;
use crate::state::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    log!("mandelbrot visualizer!");

    util::spawn_local(async move {
      let canvas = util::get_canvas()
          .map_err(|e| JsValue::from_str(&format!("{e:?}"))).unwrap();
      // ensure the canvas is the expected dimensions
      canvas.set_height(CANVAS_HEIGHT);
      canvas.set_width(CANVAS_WIDTH);

      let cxt = util::get_context(&canvas).unwrap();

      // create a channel to collect click events
      let (s, mut r) = channel::<(EventType,i32,i32)>(10);

      // when the user clicks the canvas, send the location into the channel
      let mut s1 = s.clone();
      let click_handler: Closure<dyn FnMut(web_sys::PointerEvent)>
        = Closure::new(move |evt: web_sys::PointerEvent| {
            s1.try_send((EventType::Click, evt.offset_x(), evt.offset_y())).unwrap();
          });
      canvas.set_onclick(Some(click_handler.as_ref().unchecked_ref()));

      // when the mouse moves over the canvas, send the location into the channel
      let mut s2 = s.clone();
      let mouse_move_handler: Closure<dyn FnMut(web_sys::MouseEvent)>
        = Closure::new(move |evt: web_sys::MouseEvent| {
            s2.try_send((EventType::Move, evt.offset_x(), evt.offset_y())).unwrap();
          });
      canvas.set_onmousemove(Some(mouse_move_handler.as_ref().unchecked_ref()));

      // set the starting view area
      let mut coords = Rect {
        right: 0.5,
        left: -2.0,
        top: 1.25,
        bottom: -1.25,
      };

      // render the image
      let mut image_data = draw::render(&coords);

      // draw to the canvas
      cxt.put_image_data(&image_data, 0.0, 0.0).unwrap();

      let mut state = State::NotSelecting;
      let mut x0 = 0;
      let mut y0 = 0;

      let h = CANVAS_HEIGHT as f64;
      let w = CANVAS_WIDTH as f64;

      while let Some((event_type, x, y)) = r.next().await {
        match (event_type, state) {

          // remember where the click happened and transition to Selecting state
          (EventType::Click, State::NotSelecting) => {
            x0 = x;
            y0 = y;
            state = State::Selecting;
          },

          // do nothing during movement when not in selecting state
          (EventType::Move, State::NotSelecting) => { },

          // remap complex plane to the canvas based on selected area and then rerender
          (EventType::Click, State::Selecting) => {
            // force a square selection
            let x = x0 + (if x > x0 { 1 } else { -1 } * if y > y0 { 1 } else { -1 }) * (y - y0);

            // calculate the new complex coordinates for the canvas area
            let temp_x = (x0 as f64 / w) * (coords.right - coords.left) + coords.left;
            let temp_y = ((h - y0 as f64) / h) * (coords.top - coords.bottom) + coords.bottom;
            coords.right = (x as f64 / w) * (coords.right - coords.left) + coords.left;
            coords.bottom = ((h - y as f64) / h) * (coords.top - coords.bottom) + coords.bottom;
            coords.left = temp_x;
            coords.top = temp_y;

            // reverse if necessary so directions stay consistent
            if coords.left > coords.right {
              std::mem::swap(&mut coords.left, &mut coords.right);
            }
            if coords.bottom > coords.top {
              std::mem::swap(&mut coords.bottom, &mut coords.top);
            }

            // re-render the image
            image_data = draw::render(&coords);

            // draw image to canvas
            cxt.put_image_data(&image_data, 0.0, 0.0).unwrap();

            state = State::NotSelecting;
          },

          // draw the rendered image and a square for the selection area
          (EventType::Move, State::Selecting) => {
            // force a square selection
            let x = x0 + (if x > x0 { 1 } else { -1 } * if y > y0 { 1 } else { -1 }) * (y - y0);

            // draw the currently rendered image
            cxt.put_image_data(&image_data, 0.0, 0.0).unwrap();

            // draw the selection box on top
            cxt.set_line_width(2.0);
            cxt.set_stroke_style(&JsValue::from_str("red"));
            cxt.begin_path();
            cxt.move_to(x0 as f64, y0 as f64);
            cxt.line_to(x0 as f64, y as f64);
            cxt.line_to(x as f64, y as f64);
            cxt.line_to(x as f64, y0 as f64);
            cxt.line_to(x0 as f64,y0 as f64);
            cxt.stroke();
          },
        }
      }
    });

    Ok(())
}
