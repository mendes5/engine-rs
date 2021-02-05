use core::JsRuntime;
use core::ModuleSpecifier;
use core::RuntimeOptions;
use futures;
use std::path::Path;

fn main() {
  let mut runtime = JsRuntime::new(RuntimeOptions {
    module_loader: Some(std::rc::Rc::new(core::FsModuleLoader)),
    ..Default::default()
  });

  let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/javascript/modules/index.js");
  let main_module = ModuleSpecifier::resolve_url_or_path(&js_path.to_string_lossy()).expect("Module path not found");
  let mod_id = futures::executor::block_on(runtime.load_module(&main_module, None)).expect("Failed to load");
  futures::executor::block_on(runtime.mod_evaluate(mod_id)).unwrap();
}
