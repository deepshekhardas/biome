//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttrName;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttrName {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssAttrName> for FormatAnyCssAttrName {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssAttrName> for FormatAnyCssAttrName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttrName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttrName::CssBogusAttrName(node) => node.format().fmt(f),
            AnyCssAttrName::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
        }
    }
}
