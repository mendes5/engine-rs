use anyhow::anyhow;
use core::json_op_sync;
use core::JsRuntime;
use serde_json::Value;

fn main() {
  let mut runtime = JsRuntime::new(Default::default());

  runtime.register_op(
    "op_sum",
    json_op_sync(|_state, json, zero_copy| {
      if !zero_copy.is_empty() {
        Err(anyhow!("Expected exactly one argument"))
      } else if !json.is_array() {
        Err(anyhow!("Argument is not of type array"))
      } else if !json
        .as_array()
        .unwrap()
        .iter()
        .all(|value| value.is_number())
      {
        Err(anyhow!("Argument is not array of numbers"))
      } else {
        let sum = json
          .as_array()
          .unwrap()
          .iter()
          .fold(0.0, |a, v| a + v.as_f64().unwrap());

        Ok(Value::from(sum))
      }
    }),
  );

  runtime
    .execute(
      "<usage>",
      r#"
Deno.core.ops();

const arr = [1, 2, 3];
Deno.core.print("The sum of ");
Deno.core.print(arr);
Deno.core.print(" is ");
Deno.core.print(Deno.core.jsonOpSync('op_sum', arr));
Deno.core.print("  ");
"#,
    )
    .unwrap();
}
