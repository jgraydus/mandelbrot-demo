use futures::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use web_sys;

// make printing a console log move convenient
macro_rules! log {
  ( $( $t:tt )* ) => {
    web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

#[derive(Debug)]
pub enum Error {
  WindowNotAvailable,
  DocumentNotAvailable,
  CanvasNotAvailable,
  Context2dNotAvailable,
  ResetButtonNotAvailable,
}

pub fn get_canvas() -> Result<web_sys::HtmlCanvasElement,Error> {
  let window = web_sys::window().ok_or(Error::WindowNotAvailable)?;
  let document = window.document().ok_or(Error::DocumentNotAvailable)?;
  document
    .get_element_by_id("canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_e| Error::CanvasNotAvailable)
}

pub fn get_context(
  canvas: &web_sys::HtmlCanvasElement
) -> Result<web_sys::CanvasRenderingContext2d,Error> {
  canvas
    .get_context("2d")
    .map_err(|_e| Error::Context2dNotAvailable)?
    .ok_or(Error::Context2dNotAvailable)?
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .map_err(|_e| Error::Context2dNotAvailable)
}

pub fn get_reset_button() -> Result<web_sys::HtmlButtonElement,Error> {
  let window = web_sys::window().ok_or(Error::WindowNotAvailable)?;
  let document = window.document().ok_or(Error::DocumentNotAvailable)?;
  document
    .get_element_by_id("reset_button")
    .unwrap()
    .dyn_into::<web_sys::HtmlButtonElement>()
    .map_err(|_e| Error::ResetButtonNotAvailable)
}

pub fn spawn_local<F>(future: F) 
where
  F: Future<Output = ()> + 'static
{
    wasm_bindgen_futures::spawn_local(future);
}

