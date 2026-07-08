use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{CssCounterStyleAtRuleDeclarator, CssCounterStyleAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCounterStyleAtRuleDeclarator;

impl FormatNodeRule<CssCounterStyleAtRuleDeclarator> for FormatCssCounterStyleAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssCounterStyleAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssCounterStyleAtRuleDeclaratorFields {
            counter_style_token,
            name,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&counter_style_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve()
            ]
        )
    }
}
