
use core::JsRuntime;

fn main() {
  let mut runtime = JsRuntime::new(Default::default());

  runtime
    .execute(
      "<init>",
      r#"
Deno.core.print("Hello world from v8");
"#,
    )
    .unwrap();
}