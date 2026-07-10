//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssUrlModifier;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssUrlModifier {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssUrlModifier> for FormatAnyCssUrlModifier {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssUrlModifier> for FormatAnyCssUrlModifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssUrlModifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssUrlModifier::CssBogusUrlModifier(node) => node.format().fmt(f),
            AnyCssUrlModifier::CssFunction(node) => node.format().fmt(f),
            AnyCssUrlModifier::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
        }
    }
}
