//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssQueryFeatureValue;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssQueryFeatureValue {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssQueryFeatureValue> for FormatAnyCssQueryFeatureValue {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssQueryFeatureValue> for FormatAnyCssQueryFeatureValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssQueryFeatureValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssQueryFeatureValue::AnyCssDimension(node) => node.format().fmt(f),
            AnyCssQueryFeatureValue::AnyCssFunction(node) => node.format().fmt(f),
            AnyCssQueryFeatureValue::CssIdentifier(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssQueryFeatureValue::CssNumber(node) => node.format().fmt(f),
            AnyCssQueryFeatureValue::CssRatio(node) => node.format().fmt(f),
            AnyCssQueryFeatureValue::ScssExpression(node) => node.format().fmt(f),
            AnyCssQueryFeatureValue::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
            AnyCssQueryFeatureValue::ScssInterpolation(node) => node.format().fmt(f),
            AnyCssQueryFeatureValue::ScssVariable(node) => node.format().fmt(f),
        }
    }
}
