use core::JsRuntime;

fn main() {
  let mut runtime = JsRuntime::new(Default::default());

  runtime
    .execute(
      "<init>",
      r#"
      const module = new WebAssembly.Module(new Uint8Array([
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x07, 0x01,
        0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07,
        0x07, 0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09, 0x01,
        0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b
      ]));
      const instance = new WebAssembly.Instance(module);

      const a = instance.exports.add(2, 2);
      const b = instance.exports.add(4, 4);
      const c = instance.exports.add(a, b);

      Deno.core.print("'C' calculated from WebAssembly is: " + c);
"#,
    )
    .unwrap();
}
