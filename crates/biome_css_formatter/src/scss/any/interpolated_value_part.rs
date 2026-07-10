//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssInterpolatedValuePart;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssInterpolatedValuePart {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyScssInterpolatedValuePart> for FormatAnyScssInterpolatedValuePart {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyScssInterpolatedValuePart> for FormatAnyScssInterpolatedValuePart {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssInterpolatedValuePart, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssInterpolatedValuePart::AnyCssDimension(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyScssInterpolatedValuePart::CssNumber(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::ScssInterpolation(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::ScssNamespacedVariable(node) => node.format().fmt(f),
            AnyScssInterpolatedValuePart::ScssVariable(node) => node.format().fmt(f),
        }
    }
}
