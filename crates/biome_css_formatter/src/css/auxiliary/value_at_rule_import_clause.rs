use crate::prelude::*;
use biome_css_syntax::{CssValueAtRuleImportClause, CssValueAtRuleImportClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleImportClause;
impl FormatNodeRule<CssValueAtRuleImportClause> for FormatCssValueAtRuleImportClause {
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleImportClause,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssValueAtRuleImportClauseFields {
            specifiers,
            from_token,
            source,
        } = node.as_fields();
        let formatted_source = source.format().with_case(CssCase::Preserve);

        write!(
            f,
            [
                specifiers.format(),
                space(),
                from_token.format().with_case(CssCase::Lowercase),
                space(),
                formatted_source
            ]
        )
    }
}
