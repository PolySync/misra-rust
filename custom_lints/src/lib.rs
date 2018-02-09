#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]
#![feature(macro_vis_matcher)]

extern crate syntax;

#[macro_use]
extern crate rustc;
extern crate rustc_plugin;
use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass,
                  EarlyLintPassObject, LintArray};
use rustc_plugin::Registry;
use syntax::ast;

declare_lint!(UNION_LINT, Forbid, "Disallow 'union' keyword.");
declare_lint!(UNSAFE_LINT, Forbid, "Disallow 'unsafe' keyword.");

struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(UNION_LINT, UNSAFE_LINT)
    }
}

impl EarlyLintPass for Pass {
    fn check_item(&mut self, cx: &EarlyContext, it: &ast::Item) {
        if let ast::ItemKind::Union(_,_) = it.node {
            cx.span_lint(UNION_LINT, it.span, "keyword 'union' disallowed");
        }
    }

    fn check_expr(&mut self, cx: &EarlyContext, ex: &ast::Expr) {
        if let ast::ExprKind::Block(ref block) = ex.node {
            if let ast::BlockCheckMode::Unsafe(ast::UnsafeSource::UserProvided) = block.rules {
                    cx.span_lint(UNION_LINT, ex.span, "keyword 'unsafe' disallowed");
            }
        }
    }
}

#[plugin_registrar]
pub fn register_plugins(reg: &mut Registry) {
    reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
}
