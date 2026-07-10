//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDeclarationName;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDeclarationName {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssDeclarationName> for FormatAnyCssDeclarationName {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssDeclarationName> for FormatAnyCssDeclarationName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDeclarationName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDeclarationName::AnyCssDashedIdentifier(node) => node.format().fmt(f),
            AnyCssDeclarationName::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
            AnyCssDeclarationName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
            AnyCssDeclarationName::TwValueThemeReference(node) => node.format().fmt(f),
        }
    }
}
