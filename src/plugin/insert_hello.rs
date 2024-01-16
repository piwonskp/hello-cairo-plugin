
use cairo_lang_defs::patcher::RewriteNode;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_utils::unordered_hash_map::UnorderedHashMap;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::ast::FunctionWithBody;
use cairo_lang_defs::plugin::PluginDiagnostic;


pub enum MaybeRewritten {
    Some(RewriteNode),
    None(RewriteNode),
}

type HandlingResult = (MaybeRewritten, Vec<PluginDiagnostic>);

pub fn insert_hello(
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
