use core::JsRuntime;

fn main() {
  let mut runtime = JsRuntime::new(Default::default());

  runtime
    .execute(
      "<init>",
      r#"
Deno.core.ops();
Deno.core.registerErrorClass('Error', Error);

try {
  const x = undefined;
  x.y();
} catch(e) {
  Deno.core.print('Exception:');
  Deno.core.print(e);
}

"#,
    )
    .unwrap();
}
