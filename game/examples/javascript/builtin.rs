
use core::JsRuntime;

fn main() {
  let mut runtime = JsRuntime::new(Default::default());

  runtime
    .execute(
      "<init>",
      r#"
const toLog = [
      SharedArrayBuffer,
      ArrayBuffer,
      Float64Array,
      WebAssembly,
      eval,
      Math,
      RegExp,
      Atomics,
      BigInt,
      Date,
      JSON,
      Map,
      Reflect,
];

toLog.forEach((item) => {
  runtime.print(item);
  runtime.print("\n");
});

"#,
    )
    .unwrap();
}