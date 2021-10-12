// use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
  pub fn new() -> Universe {
    let width = 64;
    let height = 64;

    let cells = (0..width * height)
      .map(|i| {
        if i % 2 == 0 || i % 7 == 0 {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    Universe {
      width,
      height,
      cells,
    }
  }

  pub fn get_width(universe: Universe) -> u32 {
    universe.width
  }
}

use serde::{Deserialize, Serialize};
use core::num;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Example {
  pub field1: HashMap<u32, String>,
  pub field2: Vec<Vec<f32>>,
  pub field3: [f32; 4],
}

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
  let mut field1 = HashMap::new();
  field1.insert(0, String::from("ex"));
  let example = Example {
    field1,
    field2: vec![vec![1., 2.], vec![3., 4.]],
    field3: [1., 2., 3., 4.],
  };

  JsValue::from_serde(&example).unwrap()
}

#[wasm_bindgen]
pub fn receive_example_from_js(val: &JsValue) -> JsValue {
  let example: Example = val.into_serde().unwrap();
  println!("{:?}", &example.field1);
  JsValue::from_serde(&example).unwrap()
}

// #[derive(Serialize, Deserialize)]
// pub struct Person {
//   pub name: String,
//   pub parent: Person,
// }

#[wasm_bindgen]
pub fn pass_value_to_js() -> Result<JsValue, JsValue> {
  serde_wasm_bindgen::to_value(vec![3., 4.])
}

// #[wasm_bindgen]
// pub fn get_value_from_js(value: JsValue) -> Result<(), JsValue> {
//   let value: SomeSupportedRustType =
//     serde_wasm_bindgen::from_value(value)?;
//   // ...
//   Ok(())
// }
