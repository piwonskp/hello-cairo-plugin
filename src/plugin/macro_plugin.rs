use cairo_lang_defs::plugin::{
    DynGeneratedFileAuxData, GeneratedFileAuxData, MacroPlugin, PluginGeneratedFile, PluginResult
};
use cairo_lang_syntax::node::ast::FunctionWithBody;
use cairo_lang_defs::patcher::PatchBuilder;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{ast, Terminal};
use smol_str::SmolStr;
use cairo_lang_syntax::node::helpers::QueryAttrs;

use crate::plugin::insert_hello::{insert_hello, MaybeRewritten};


#[derive(Debug, Default)]
pub struct HelloPlugin;

// impl HelloPlugin {
    fn _handle_function(db: &dyn SyntaxGroup, func_ast: &FunctionWithBody) -> PluginResult {
        let (maybe_rewriten_func, implicit_diagnostics) =
            insert_hello(db, func_ast.clone());
        return if let MaybeRewritten::Some(rewritten_func) = maybe_rewriten_func {
            let func_name = func_ast.declaration(db).name(db).text(db);
            let mut builder = PatchBuilder::new(db);
            builder.add_modified(rewritten_func);
            PluginResult {
                code: Some(PluginGeneratedFile {
                    name: func_name.clone(),
                    content: builder.code,
                    aux_data: Some(DynGeneratedFileAuxData::new(AuxData {
                        elements: vec![func_name],
                    })),
                    diagnostics_mappings: vec![]
                }),
                diagnostics: implicit_diagnostics,
                remove_original_item: true,
            }
        } else {
            PluginResult {
                code: None,
                diagnostics: implicit_diagnostics,
                remove_original_item: false,
            }
        };
    }
// }


impl MacroPlugin for HelloPlugin {
    fn generate_code(&self, db: &dyn SyntaxGroup, item_ast: ast::Item) -> PluginResult {
        // Register functions to use for item of choice
        match &item_ast {
            ast::Item::FreeFunction(function) if function.has_attr(db, "hello") => _handle_function(db, function),
            _ => PluginResult::default(),
        }
    }

    fn declared_attributes(&self) -> Vec<String> {
        vec![
            "hello".to_string()
        ]
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct AuxData {
    /// A list of elements that were processed by the plugin.
    pub elements: Vec<SmolStr>,
}

impl GeneratedFileAuxData for AuxData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn eq(&self, other: &dyn GeneratedFileAuxData) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}