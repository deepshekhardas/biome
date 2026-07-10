use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{CssColorProfileAtRuleDeclarator, CssColorProfileAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssColorProfileAtRuleDeclarator;

impl FormatNodeRule<CssColorProfileAtRuleDeclarator> for FormatCssColorProfileAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssColorProfileAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssColorProfileAtRuleDeclaratorFields {
            color_profile_token,
            name,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&color_profile_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve()
            ]
        )
    }
}
