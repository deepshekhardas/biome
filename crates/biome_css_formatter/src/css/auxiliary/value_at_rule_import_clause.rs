use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{
    AnyCssValueAtRuleImportSource, CssValueAtRuleImportClause, CssValueAtRuleImportClauseFields,
};
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
        let source = source?;
        let formatted_source = format_with(|f| match &source {
            AnyCssValueAtRuleImportSource::CssIdentifier(source) => {
                format_css_identifier(source).preserve().fmt(f)
            }
            AnyCssValueAtRuleImportSource::CssString(source) => source.format().fmt(f),
        });

        write!(
            f,
            [
                specifiers.format(),
                space(),
                format_css_token(&from_token?).lowercase(),
                space(),
                formatted_source
            ]
        )
    }
}
