//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssValueAtRuleImportSource;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssValueAtRuleImportSource {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssValueAtRuleImportSource> for FormatAnyCssValueAtRuleImportSource {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssValueAtRuleImportSource> for FormatAnyCssValueAtRuleImportSource {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssValueAtRuleImportSource, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssValueAtRuleImportSource::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssValueAtRuleImportSource::CssString(node) => node.format().fmt(f),
        }
    }
}
