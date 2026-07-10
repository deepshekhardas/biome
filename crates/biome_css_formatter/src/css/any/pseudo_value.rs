//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPseudoValue;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPseudoValue {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssPseudoValue> for FormatAnyCssPseudoValue {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssPseudoValue> for FormatAnyCssPseudoValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPseudoValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPseudoValue::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
            AnyCssPseudoValue::CssString(node) => node.format().fmt(f),
            AnyCssPseudoValue::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
            AnyCssPseudoValue::ScssInterpolatedString(node) => node.format().fmt(f),
            AnyCssPseudoValue::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}
