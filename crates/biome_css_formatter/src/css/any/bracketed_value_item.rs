//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssBracketedValueItem;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssBracketedValueItem {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssBracketedValueItem> for FormatAnyCssBracketedValueItem {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssBracketedValueItem> for FormatAnyCssBracketedValueItem {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssBracketedValueItem, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssBracketedValueItem::AnyCssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssBracketedValueItem::AnyScssExpression(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssBracketedValueItem::CssGenericDelimiter(node) => node.format().fmt(f),
        }
    }
}
