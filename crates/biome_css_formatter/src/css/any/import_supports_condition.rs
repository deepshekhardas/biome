//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssImportSupportsCondition;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssImportSupportsCondition {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssImportSupportsCondition> for FormatAnyCssImportSupportsCondition {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssImportSupportsCondition> for FormatAnyCssImportSupportsCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssImportSupportsCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssImportSupportsCondition::AnyCssSupportsCondition(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssImportSupportsCondition::CssDeclaration(node) => node.format().fmt(f),
        }
    }
}
