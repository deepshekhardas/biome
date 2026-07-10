//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssExpression;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssExpression {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyScssExpression> for FormatAnyScssExpression {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyScssExpression> for FormatAnyScssExpression {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssExpression::AnyCssValue(node) => node.format().with_case(self.case).fmt(f),
            AnyScssExpression::ScssBinaryExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssInterpolation(node) => node.format().fmt(f),
            AnyScssExpression::ScssListExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssMapExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssParenthesizedExpression(node) => node.format().fmt(f),
            AnyScssExpression::ScssUnaryExpression(node) => node.format().fmt(f),
        }
    }
}
