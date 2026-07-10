use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{CssPropertyAtRuleDeclarator, CssPropertyAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPropertyAtRuleDeclarator;

impl FormatNodeRule<CssPropertyAtRuleDeclarator> for FormatCssPropertyAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssPropertyAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPropertyAtRuleDeclaratorFields {
            property_token,
            name,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&property_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve()
            ]
        )
    }
}
