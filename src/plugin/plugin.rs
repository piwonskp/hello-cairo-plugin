use std::fmt;
use anyhow::Result;

use cairo_lang_defs::plugin::{
    DynGeneratedFileAuxData, GeneratedFileAuxData, MacroPlugin, PluginGeneratedFile, PluginResult, InlineMacroExprPlugin
};
use cairo_lang_defs::plugin::PluginSuite;
use cairo_lang_defs::patcher::PatchBuilder;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_utils::unordered_hash_map::UnorderedHashMap;
use scarb::compiler::plugin::builtin::BuiltinStarkNetPlugin;
use cairo_lang_syntax::node::ast::FunctionWithBody;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{ast, Terminal, TypedSyntaxNode};
use cairo_lang_defs::patcher::RewriteNode;
use cairo_lang_defs::plugin::PluginDiagnostic;

use camino::Utf8Path;

use scarb::compiler::plugin::{CairoPlugin, CairoPluginInstance};
use scarb::core::{PackageId, PackageName, SourceId};
use semver::Version;
use smol_str::SmolStr;

use url::Url;


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

pub enum MaybeRewritten {
    Some(RewriteNode),
    None(RewriteNode),
}


type HandlingResult = (MaybeRewritten, Vec<PluginDiagnostic>);

pub fn handle_function(
    db: &dyn SyntaxGroup,
    function_body: FunctionWithBody,
) -> HandlingResult {
    let diagnostics: Vec<PluginDiagnostic> = vec![];
    let attributes = function_body.attributes(db);
    let custom_implict_index = attributes
        .elements(db)
        .iter()
        .position(|attr| attr.attr(db).as_syntax_node().get_text_without_trivia(db) == "hello");
    if custom_implict_index.is_none() {
        return (MaybeRewritten::None(RewriteNode::from_ast(&function_body)), diagnostics)
    }

    let body = function_body.body(db);
    let rewritten_body = RewriteNode::interpolate_patched("
    debug::print_felt252('Hello, $func_name$!');
    $body$
    ", &UnorderedHashMap::from([
        ("body".to_string(), RewriteNode::from_ast(&body)),
        ("func_name".to_string(), RewriteNode::from(function_body.declaration(db).name(db).as_syntax_node()))
        ]));

    let mut rewritten_attributes = RewriteNode::from_ast(&attributes);

    if let Some(index) = custom_implict_index {
        rewritten_attributes
            .modify(db)
            .children
            .as_mut()
            .unwrap()
            .remove(index);
    }

    let rewritten_function = RewriteNode::interpolate_patched(
        "
        $attributes$
        $func_decl$ {
            $body$
        }
        ",
        &UnorderedHashMap::from([
            ("attributes".to_string(), rewritten_attributes),
            ("func_decl".to_string(), RewriteNode::from_ast(&function_body.declaration(db))),
            ("body".to_string(), rewritten_body),
        ]),
    );

    (MaybeRewritten::Some(rewritten_function), diagnostics)
}


#[derive(Debug, Default)]
pub struct HelloPlugin;

impl HelloPlugin {
    fn _handle_function(&self, db: &dyn SyntaxGroup, func_ast: &FunctionWithBody) -> PluginResult {
        let (maybe_rewriten_func, implicit_diagnostics) =
            handle_function(db, func_ast.clone());
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
}

impl MacroPlugin for HelloPlugin {
    fn generate_code(&self, db: &dyn SyntaxGroup, item_ast: ast::Item) -> PluginResult {
        match &item_ast {
            ast::Item::FreeFunction(function) if function.has_attr(db, "hello") => self._handle_function(db, function),
            _ => PluginResult::default(),
        }
    }

    fn declared_attributes(&self) -> Vec<String> {
        vec![
            "hello".to_string()
        ]
    }
}


// Target the plugin located on a local filesystem. Useful for development
struct LocalPlugin;
impl CairoPlugin for LocalPlugin {
    fn id(&self) -> PackageId {
        PackageId::new(
            PackageName::new("hello"),
            Version::parse("0.0.0").unwrap(),
            SourceId::for_path(&(Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("Scarb.toml"))).unwrap(),
        )
    }

    fn instantiate(&self) -> Result<Box<dyn CairoPluginInstance>> {
        Ok(Box::new(PluginInstance))
    }
}


// Target the plugin located in remote repository
struct GitPlugin;
impl CairoPlugin for GitPlugin {
    fn id(&self) -> PackageId {
        PackageId::new(
            PackageName::new("hello"),
            Version::parse("0.0.0").unwrap(),
            SourceId::for_git(
                        &Url::parse("https://github.com/piwonskp/hello-cairo-plugin").unwrap(),
                        &scarb::core::GitReference::DefaultBranch,
                    )
                    .unwrap(),
        )
    }

    fn instantiate(&self) -> Result<Box<dyn CairoPluginInstance>> {
        Ok(Box::new(PluginInstance))
    }
}


// Target the plugin inside the compiler, unsupported yet (v2.4.0)
// See https://github.com/software-mansion/scarb/issues/119, https://github.com/software-mansion/scarb/discussions/227
struct InCompilerPlugin;
impl CairoPlugin for InCompilerPlugin {
    fn id(&self) -> PackageId {
        PackageId::new(
            PackageName::new("hello"),
            Version::parse("0.0.0").unwrap(),
            SourceId::for_std(),
        )
    }

    fn instantiate(&self) -> Result<Box<dyn CairoPluginInstance>> {
        Ok(Box::new(PluginInstance))
    }
}


struct PluginInstance;
impl CairoPluginInstance for PluginInstance {
    fn plugin_suite(&self) -> PluginSuite {
        let mut suite = PluginSuite::default();
        suite.add_plugin::<HelloPlugin>();
        suite
    }
}

pub struct CairoPluginRepository(pub scarb::compiler::plugin::CairoPluginRepository);

impl CairoPluginRepository {
    pub fn new() -> Self {
        let mut repo = scarb::compiler::plugin::CairoPluginRepository::empty();

        repo.add(Box::new(BuiltinStarkNetPlugin)).unwrap();
        repo.add(Box::new(LocalPlugin)).unwrap();
        repo.add(Box::new(GitPlugin)).unwrap();
        repo.add(Box::new(InCompilerPlugin)).unwrap();

        Self(repo)
    }
}

impl Default for CairoPluginRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CairoPluginRepository> for scarb::compiler::plugin::CairoPluginRepository {
    fn from(val: CairoPluginRepository) -> Self {
        val.0
    }
}

impl fmt::Debug for CairoPluginRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
