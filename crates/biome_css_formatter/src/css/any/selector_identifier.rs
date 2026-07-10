//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSelectorIdentifier;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSelectorIdentifier {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssSelectorIdentifier> for FormatAnyCssSelectorIdentifier {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssSelectorIdentifier> for FormatAnyCssSelectorIdentifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSelectorIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSelectorIdentifier::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssSelectorIdentifier::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}
