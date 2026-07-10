//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyTwUtilityName;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTwUtilityName {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyTwUtilityName> for FormatAnyTwUtilityName {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyTwUtilityName> for FormatAnyTwUtilityName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyTwUtilityName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyTwUtilityName::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
            AnyTwUtilityName::TwFunctionalUtilityName(node) => node.format().fmt(f),
        }
    }
}
