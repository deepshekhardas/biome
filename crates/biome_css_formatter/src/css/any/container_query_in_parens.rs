//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerQueryInParens;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerQueryInParens {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssContainerQueryInParens> for FormatAnyCssContainerQueryInParens {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssContainerQueryInParens> for FormatAnyCssContainerQueryInParens {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerQueryInParens, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerQueryInParens::AnyCssValue(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssContainerQueryInParens::CssContainerQueryInParens(node) => node.format().fmt(f),
            AnyCssContainerQueryInParens::CssContainerScrollStateQueryInParens(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerQueryInParens::CssContainerSizeFeatureInParens(node) => {
                node.format().fmt(f)
            }
            AnyCssContainerQueryInParens::CssContainerStyleQueryInParens(node) => {
                node.format().fmt(f)
            }
        }
    }
}
