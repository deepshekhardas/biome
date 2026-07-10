//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSupportsOrCombinableCondition;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSupportsOrCombinableCondition {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssSupportsOrCombinableCondition>
    for FormatAnyCssSupportsOrCombinableCondition
{
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssSupportsOrCombinableCondition> for FormatAnyCssSupportsOrCombinableCondition {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssSupportsOrCombinableCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssSupportsOrCombinableCondition::AnyCssSupportsInParens(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssSupportsOrCombinableCondition::CssSupportsOrCondition(node) => {
                node.format().fmt(f)
            }
        }
    }
}
