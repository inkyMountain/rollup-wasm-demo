import rust from "./rust/Cargo.toml";
rust().then(({ Universe, receive_example_from_js, send_example_to_js, pass_value_to_js }) => {
  // const universe = Universe.new();
  // console.log('universe ==========>', universe)
  // const width = Universe.get_width(universe)
  // console.log('width ==========>', width)
  const xxx = send_example_to_js();
  console.log("example ==========>", xxx);

  // Get the example object from wasm.
  const example = send_example_to_js();

  // Add another "Vec" element to the end of the "Vec<Vec<f32>>"
  example.field2.push([5, 6]);

  // Send the example object back to wasm.
  receive_example_from_js(example);
  const serdeValue = pass_value_to_js()
  console.log('serdeValue ==========>', serdeValue)
});
