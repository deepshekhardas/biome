//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssUseNamespace;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssUseNamespace {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyScssUseNamespace> for FormatAnyScssUseNamespace {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyScssUseNamespace> for FormatAnyScssUseNamespace {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssUseNamespace, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssUseNamespace::CssIdentifier(node) => node.format().with_case(self.case).fmt(f),
            AnyScssUseNamespace::ScssUseAllNamespace(node) => node.format().fmt(f),
        }
    }
}
