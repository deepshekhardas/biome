//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSupportsCondition;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSupportsCondition {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssSupportsCondition> for FormatAnyCssSupportsCondition {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssSupportsCondition> for FormatAnyCssSupportsCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSupportsCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSupportsCondition::AnyCssSupportsInParens(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssSupportsCondition::CssBogusSupportsCondition(node) => node.format().fmt(f),
            AnyCssSupportsCondition::CssSupportsAndCondition(node) => node.format().fmt(f),
            AnyCssSupportsCondition::CssSupportsNotCondition(node) => node.format().fmt(f),
            AnyCssSupportsCondition::CssSupportsOrCondition(node) => node.format().fmt(f),
        }
    }
}
