//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSupportsInParens;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSupportsInParens {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssSupportsInParens> for FormatAnyCssSupportsInParens {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssSupportsInParens> for FormatAnyCssSupportsInParens {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSupportsInParens, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSupportsInParens::AnyCssValue(node) => node.format().with_case(self.case).fmt(f),
            AnyCssSupportsInParens::CssSupportsConditionInParens(node) => node.format().fmt(f),
            AnyCssSupportsInParens::CssSupportsFeatureDeclaration(node) => node.format().fmt(f),
            AnyCssSupportsInParens::CssSupportsFeatureSelector(node) => node.format().fmt(f),
            AnyCssSupportsInParens::ScssSupportsInterpolatedCondition(node) => node.format().fmt(f),
        }
    }
}
