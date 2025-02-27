use crate::types::TransformConfig;
use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ImportDecl, ImportDefaultSpecifier, ImportSpecifier, ModuleDecl, ModuleItem, Str,
};

pub fn transform_perfixreact(
  module: &mut swc_ecma_ast::Module,
  project_config: &TransformConfig,
  _origin_code: &str,
) {
  let mut need_add = true;
  if project_config.react_runtime.unwrap_or(false) {
    for item in &module.body {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
        let source = &*var.src.value;
        if source == "react" {
          for specifier in &var.specifiers {
            match specifier {
              ImportSpecifier::Named(ref _s) => {}
              ImportSpecifier::Default(ref s) => {
                if format!("{}", s.local.sym) == *"React".to_string() {
                  need_add = false;
                }
              }
              ImportSpecifier::Namespace(ref _s) => {}
            }
          }
        }
      }
    }

    if need_add {
      let body = &mut module.body;
      let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![swc_ecma_ast::ImportSpecifier::Default(
          ImportDefaultSpecifier {
            span: DUMMY_SP,
            local: swc_ecma_ast::Ident {
              span: DUMMY_SP,
              sym: JsWord::from("React"),
              optional: false,
            },
          },
        )],
        src: Str {
          span: DUMMY_SP,
          value: JsWord::from("react"),
          has_escape: false,
          kind: Default::default(),
        },
        type_only: false,
        asserts: None,
      }));
      body.insert(0, dec);
    }
  }
}
