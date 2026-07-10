//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssExpressionItem;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssExpressionItem {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyScssExpressionItem> for FormatAnyScssExpressionItem {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyScssExpressionItem> for FormatAnyScssExpressionItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssExpressionItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssExpressionItem::AnyCssValue(node) => node.format().with_case(self.case).fmt(f),
            AnyScssExpressionItem::CssDeclarationImportant(node) => node.format().fmt(f),
            AnyScssExpressionItem::CssGenericDelimiter(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssArbitraryArgument(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssBinaryExpression(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssInterpolation(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssKeywordArgument(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssListExpression(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssMapExpression(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssParenthesizedExpression(node) => node.format().fmt(f),
            AnyScssExpressionItem::ScssUnaryExpression(node) => node.format().fmt(f),
        }
    }
}
