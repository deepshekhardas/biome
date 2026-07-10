//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssModuleMember;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssModuleMember {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyScssModuleMember> for FormatAnyScssModuleMember {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyScssModuleMember> for FormatAnyScssModuleMember {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssModuleMember, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssModuleMember::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
            AnyScssModuleMember::ScssVariable(node) => node.format().fmt(f),
        }
    }
}
