//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssSyntaxSingleComponent;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssSyntaxSingleComponent {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssSyntaxSingleComponent> for FormatAnyCssSyntaxSingleComponent {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssSyntaxSingleComponent> for FormatAnyCssSyntaxSingleComponent {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssSyntaxSingleComponent, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssSyntaxSingleComponent::CssBogusSyntaxSingleComponent(node) => {
                node.format().fmt(f)
            }
            AnyCssSyntaxSingleComponent::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssSyntaxSingleComponent::CssSyntaxType(node) => node.format().fmt(f),
        }
    }
}
