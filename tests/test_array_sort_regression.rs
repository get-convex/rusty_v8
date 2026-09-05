#[test]
fn array_sort_preserves_elements_kind() {
  assert!(
    v8::icu::set_common_data_77(align_data::include_aligned!(
      align_data::Align16,
      "../third_party/icu/common/icudtl.dat"
    ))
    .is_ok()
  );
  v8::V8::set_flags_from_string(
    "--allow-natives-syntax --maglev --no-concurrent-recompilation",
  );
  v8::V8::initialize_platform(
    v8::new_unprotected_default_platform(0, false).make_shared(),
  );
  v8::V8::initialize();

  let regression =
    include_str!("../v8/test/mjsunit/regress/regress-crbug-542403045.js");
  for compiler in ["Maglev", "TurboFan"] {
    let mut isolate = v8::Isolate::new(Default::default());
    v8::scope!(let scope, &mut isolate);
    let context = v8::Context::new(scope, Default::default());
    let scope = &mut v8::ContextScope::new(scope, context);
    v8::tc_scope!(let scope, scope);
    let regression = if compiler == "TurboFan" {
      regression
        .replace("%OptimizeMaglevOnNextCall", "%OptimizeFunctionOnNextCall")
    } else {
      regression.to_owned()
    };
    let source = format!(
      "{}\n{}",
      include_str!("../v8/test/mjsunit/mjsunit.js"),
      regression
    );
    let source = v8::String::new(scope, &source).unwrap();
    let result = v8::Script::compile(scope, source, None)
      .and_then(|script| script.run(scope));
    if result.is_none() {
      let exception = scope.exception().unwrap();
      panic!("{compiler}: {}", exception.to_rust_string_lossy(scope));
    }
  }

  unsafe { v8::V8::dispose() };
  v8::V8::dispose_platform();
}
