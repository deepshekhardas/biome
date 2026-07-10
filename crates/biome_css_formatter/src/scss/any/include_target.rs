//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssIncludeTarget;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssIncludeTarget {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyScssIncludeTarget> for FormatAnyScssIncludeTarget {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyScssIncludeTarget> for FormatAnyScssIncludeTarget {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssIncludeTarget, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssIncludeTarget::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
            AnyScssIncludeTarget::ScssModuleMemberAccess(node) => node.format().fmt(f),
        }
    }
}
