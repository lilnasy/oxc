use rustc_hash::FxHashSet;

use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{
    AstNode,
    context::LintContext,
    module_record::{ExportEntry, ExportLocalName},
    rule::Rule,
};

fn require_public_doc_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Missing JSDoc for public export.")
        .with_help("Add a JSDoc comment for this exported API.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct RequirePublicDoc;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires that locally exported APIs are documented with a JSDoc block.
    ///
    /// This checks local exports in the current module (e.g. `export function foo() {}`,
    /// `export class Foo {}`, `export const bar = () => {}`, and `export { foo }`).
    /// Re-exports from other modules (e.g. `export { foo } from 'mod'`) are ignored.
    ///
    /// ### Why is this bad?
    ///
    /// Public APIs without documentation reduce maintainability and discoverability.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// export function quux() {}
    /// export const bar = () => {};
    /// function foo() {}
    /// export { foo };
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```javascript
    /// /** Docs */
    /// export function quux() {}
    /// /** Docs */
    /// export const bar = () => {};
    /// /** Docs */
    /// function foo() {}
    /// export { foo };
    /// ```
    RequirePublicDoc,
    jsdoc,
    pedantic
);

impl Rule for RequirePublicDoc {
    fn run_once(&self, ctx: &LintContext) {
        let module = ctx.module_record();

        // Collect locally exported symbol names and their export spans.
        let mut exported_symbols: Vec<(&str, Span)> = Vec::new();

        for ExportEntry { module_request, local_name, span, .. } in &module.local_export_entries {
            // Ignore re-exports from other modules
            if module_request.is_some() {
                continue;
            }

            match local_name {
                ExportLocalName::Name(name_span) | ExportLocalName::Default(name_span) => {
                    exported_symbols.push((name_span.name.as_str(), *span));
                }
                ExportLocalName::Null => {
                    // Cannot resolve anonymous default export or specifier-less cases.
                }
            }
        }

        // Deduplicate by symbol id once resolved
        let mut seen: FxHashSet<oxc_semantic::SymbolId> = FxHashSet::default();

        for (name, export_span) in exported_symbols {
            let Some(symbol_id) = ctx.scoping().get_root_binding(name) else { continue };
            if !seen.insert(symbol_id) {
                continue;
            }

            let decl_id = ctx.scoping().symbol_declaration(symbol_id);
            let decl_node = ctx.nodes().get_node(decl_id);

            if !has_any_attached_jsdoc(decl_node, ctx) {
                ctx.diagnostic(require_public_doc_diagnostic(export_span));
            }
        }
    }
}

fn has_any_attached_jsdoc<'a>(start: &AstNode<'a>, ctx: &LintContext<'a>) -> bool {
    // Walk up ancestors from the declaration node and check if any node along the way
    // has JSDoc attached. This covers common cases where docs are attached to
    // VariableDeclaration, Export*Declaration, or the Function/Class node itself.
    let mut current = start;
    loop {
        if ctx.jsdoc().get_all_by_node(ctx.nodes(), current).is_some() {
            return true;
        }

        let parent = ctx.nodes().parent_node(current.id());
        match parent.kind() {
            AstKind::Program(_) => return false,
            _ => current = parent,
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        (
            "/** Docs */\nexport function quux() {}",
            None,
            None,
        ),
        (
            "/** Docs */\nexport const bar = () => {};",
            None,
            None,
        ),
        (
            "/** Docs */\nfunction foo() {}\nexport { foo };",
            None,
            None,
        ),
        (
            "export { foo } from 'mod';",
            None,
            None,
        ),
    ];

    let fail = vec![
        (
            "export function quux() {}",
            None,
            None,
        ),
        (
            "export const bar = () => {};",
            None,
            None,
        ),
        (
            "function foo() {}\nexport { foo };",
            None,
            None,
        ),
        (
            "export default function quux() {}",
            None,
            None,
        ),
    ];

    Tester::new(RequirePublicDoc::NAME, RequirePublicDoc::PLUGIN, pass, fail).test_and_snapshot();
}
