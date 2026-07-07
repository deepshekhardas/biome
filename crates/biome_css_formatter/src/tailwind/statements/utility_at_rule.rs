use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{TwUtilityAtRule, TwUtilityAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwUtilityAtRule;
impl FormatNodeRule<TwUtilityAtRule> for FormatTwUtilityAtRule {
    fn fmt_fields(&self, node: &TwUtilityAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwUtilityAtRuleFields {
            utility_token,
            name,
            block,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&utility_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve(),
                space(),
                block.format()
            ]
        )
    }
}
