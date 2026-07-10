//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssQueryFeatureName;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssQueryFeatureName {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssQueryFeatureName> for FormatAnyCssQueryFeatureName {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssQueryFeatureName> for FormatAnyCssQueryFeatureName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssQueryFeatureName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssQueryFeatureName::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssQueryFeatureName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
            AnyCssQueryFeatureName::ScssVariable(node) => node.format().fmt(f),
        }
    }
}
