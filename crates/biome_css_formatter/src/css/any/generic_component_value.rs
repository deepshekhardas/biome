//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssGenericComponentValue;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssGenericComponentValue {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssGenericComponentValue> for FormatAnyCssGenericComponentValue {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssGenericComponentValue> for FormatAnyCssGenericComponentValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssGenericComponentValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssGenericComponentValue::AnyCssValue(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssGenericComponentValue::CssGenericDelimiter(node) => node.format().fmt(f),
        }
    }
}
