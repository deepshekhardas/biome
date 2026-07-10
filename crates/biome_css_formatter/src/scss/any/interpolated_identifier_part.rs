//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedIdentifierPart;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedIdentifierPart {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyScssInterpolatedIdentifierPart>
    for FormatAnyScssInterpolatedIdentifierPart
{
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyScssInterpolatedIdentifierPart> for FormatAnyScssInterpolatedIdentifierPart {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyScssInterpolatedIdentifierPart,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyScssInterpolatedIdentifierPart::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyScssInterpolatedIdentifierPart::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyScssInterpolatedIdentifierPart::ScssInterpolatedIdentifierHyphen(node) => {
                node.format().fmt(f)
            }
            AnyScssInterpolatedIdentifierPart::ScssInterpolation(node) => node.format().fmt(f),
        }
    }
}
