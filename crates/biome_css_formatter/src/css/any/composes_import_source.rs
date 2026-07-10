//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssComposesImportSource;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssComposesImportSource {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssComposesImportSource> for FormatAnyCssComposesImportSource {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssComposesImportSource> for FormatAnyCssComposesImportSource {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssComposesImportSource, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssComposesImportSource::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssComposesImportSource::CssString(node) => node.format().fmt(f),
        }
    }
}
