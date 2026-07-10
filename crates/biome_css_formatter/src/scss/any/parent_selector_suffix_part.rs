//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssParentSelectorSuffixPart;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssParentSelectorSuffixPart {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyScssParentSelectorSuffixPart>
    for FormatAnyScssParentSelectorSuffixPart
{
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyScssParentSelectorSuffixPart> for FormatAnyScssParentSelectorSuffixPart {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyScssParentSelectorSuffixPart,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyScssParentSelectorSuffixPart::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyScssParentSelectorSuffixPart::CssNumber(node) => node.format().fmt(f),
            AnyScssParentSelectorSuffixPart::ScssInterpolation(node) => node.format().fmt(f),
            AnyScssParentSelectorSuffixPart::ScssParentSelectorSuffixHyphen(node) => {
                node.format().fmt(f)
            }
        }
    }
}
