//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfSupportsTestCondition;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfSupportsTestCondition {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssIfSupportsTestCondition> for FormatAnyCssIfSupportsTestCondition {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssIfSupportsTestCondition> for FormatAnyCssIfSupportsTestCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfSupportsTestCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfSupportsTestCondition::AnyCssImportSupportsCondition(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssIfSupportsTestCondition::CssIfSupportsIdentifierTest(node) => {
                node.format().fmt(f)
            }
        }
    }
}
