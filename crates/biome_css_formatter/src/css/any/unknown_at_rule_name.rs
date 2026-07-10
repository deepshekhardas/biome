//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssUnknownAtRuleName;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssUnknownAtRuleName {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssUnknownAtRuleName> for FormatAnyCssUnknownAtRuleName {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssUnknownAtRuleName> for FormatAnyCssUnknownAtRuleName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssUnknownAtRuleName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssUnknownAtRuleName::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssUnknownAtRuleName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
            AnyCssUnknownAtRuleName::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}
