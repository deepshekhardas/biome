use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssValueAtRule, CssValueAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRule;
impl FormatNodeRule<CssValueAtRule> for FormatCssValueAtRule {
    fn fmt_fields(&self, node: &CssValueAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssValueAtRuleFields {
            value_token,
            clause,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&value_token?).lowercase(),
                space(),
                clause.format(),
                semicolon_token.format(),
            ]
        )
    }
}
