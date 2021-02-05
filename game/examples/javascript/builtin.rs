
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
  Deno.core.print(item);
  Deno.core.print("\n");
});

"#,
    )
    .unwrap();
}