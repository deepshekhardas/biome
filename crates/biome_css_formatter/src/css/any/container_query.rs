//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerQuery;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerQuery {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssContainerQuery> for FormatAnyCssContainerQuery {
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssContainerQuery> for FormatAnyCssContainerQuery {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssContainerQuery, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssContainerQuery::AnyCssContainerQueryInParens(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssContainerQuery::CssContainerAndQuery(node) => node.format().fmt(f),
            AnyCssContainerQuery::CssContainerNotQuery(node) => node.format().fmt(f),
            AnyCssContainerQuery::CssContainerOrQuery(node) => node.format().fmt(f),
        }
    }
}
