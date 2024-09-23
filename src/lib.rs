use cairo_lang_macro::{
    attribute_macro, Diagnostic, Diagnostics, ProcMacroResult, TokenStream,
};
use cairo_lang_diagnostics::FormattedDiagnosticEntry;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::ast::{ModuleItem, SyntaxFile};
use cairo_lang_syntax::node::TypedSyntaxNode;
use core::ops::Deref;
use regex::Regex;


fn parser_to_macro_diagnostic(entry: &FormattedDiagnosticEntry) -> Diagnostic {
    let severity = match entry.severity() {
        cairo_lang_diagnostics::Severity::Error => cairo_lang_macro::Severity::Error,
        cairo_lang_diagnostics::Severity::Warning => cairo_lang_macro::Severity::Warning,
    };
    Diagnostic {
        message: entry.message().to_string(),
        severity: severity,
    }
}

/// Insert hello at the begining of the function using fully fledged cairo parser  
#[attribute_macro]
pub fn hello(_attrs: TokenStream, token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();

    let (parsed_node, parser_diagnostics) = db.parse_virtual_with_diagnostics(token_stream);
    let formatted_diags = parser_diagnostics.format_with_severity(&db);
    let diagnostics = formatted_diags.iter().map(parser_to_macro_diagnostic);

    let module_items = SyntaxFile::from_syntax_node(&db, parsed_node)
        .items(&db)
        .deref()
        .elements(&db);
    let function = match module_items.as_slice() {
        [ModuleItem::FreeFunction(fun)] => fun,
        // Currently attribute macros are run only on functions so the code should never reach this place
        _ => panic!("hello attribute may be set on functions only"),
    };

    let instrumented_function = format!(
        "
        {} {{
            core::debug::print_felt252('Hello from ast, {}!');
            {}
        }}
        ",
        function.declaration(&db).as_syntax_node().get_text(&db),
        function
            .declaration(&db)
            .name(&db)
            .as_syntax_node()
            .get_text(&db),
        function.body(&db).as_syntax_node().get_text(&db)
    );

    ProcMacroResult::new(TokenStream::new(instrumented_function))
        .with_diagnostics(Diagnostics::new(diagnostics.collect()))
}


/// A lightweight version of hello macro using regex
#[attribute_macro]
pub fn hello_regex(_attrs: TokenStream, token_stream: TokenStream) -> ProcMacroResult {
    let fn_regex =
        Regex::new(r"^(\s*fn\s+(?<function_name>\w+)\s*\(.*?\)\s*(->\s*.*)?\{)").unwrap();
    let function = token_stream.to_string();
    let instrumented = fn_regex.replace_all(&function, |caps: &regex::Captures| {
        format!(
            "{}\n\tcore::debug::print_felt252('Hello {}!');",
            &caps[0], &caps["function_name"]
        )
    });

    ProcMacroResult::new(TokenStream::new(instrumented.to_string()))
}
