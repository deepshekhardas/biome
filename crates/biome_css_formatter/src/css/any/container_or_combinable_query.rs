//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssContainerOrCombinableQuery;
use biome_formatter::FormatRuleWithOptions;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssContainerOrCombinableQuery {
    case: CssCase,
}
impl FormatRuleWithOptions<AnyCssContainerOrCombinableQuery>
    for FormatAnyCssContainerOrCombinableQuery
{
    type Options = CssCase;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}
impl FormatRule<AnyCssContainerOrCombinableQuery> for FormatAnyCssContainerOrCombinableQuery {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssContainerOrCombinableQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssContainerOrCombinableQuery::AnyCssContainerQueryInParens(node) => {
                node.format().with_case(self.case).fmt(f)
            }
            AnyCssContainerOrCombinableQuery::CssContainerOrQuery(node) => node.format().fmt(f),
        }
    }
}
