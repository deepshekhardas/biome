//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttributeName;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttributeName {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssAttributeName> for FormatAnyCssAttributeName {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssAttributeName> for FormatAnyCssAttributeName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttributeName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttributeName::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
            AnyCssAttributeName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
        }
    }
}
