use crate::prelude::*;
use crate::utils::case::format_css_token;
use crate::utils::scss_statement_at_rule::format_scss_statement_at_rule_semicolon;
use biome_css_syntax::{ScssReturnAtRule, ScssReturnAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssReturnAtRule;

impl FormatNodeRule<ScssReturnAtRule> for FormatScssReturnAtRule {
    fn fmt_fields(&self, node: &ScssReturnAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssReturnAtRuleFields {
            return_token,
            value,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&return_token?).lowercase(),
                space(),
                value.format(),
                format_scss_statement_at_rule_semicolon(semicolon_token)
            ]
        )
    }
}
