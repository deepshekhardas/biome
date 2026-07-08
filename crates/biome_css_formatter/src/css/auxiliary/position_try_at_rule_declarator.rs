use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{CssPositionTryAtRuleDeclarator, CssPositionTryAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPositionTryAtRuleDeclarator;

impl FormatNodeRule<CssPositionTryAtRuleDeclarator> for FormatCssPositionTryAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssPositionTryAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPositionTryAtRuleDeclaratorFields {
            position_try_token,
            name,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&position_try_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve()
            ]
        )
    }
}
