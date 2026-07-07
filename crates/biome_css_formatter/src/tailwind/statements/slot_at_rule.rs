use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{TwSlotAtRule, TwSlotAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwSlotAtRule;
impl FormatNodeRule<TwSlotAtRule> for FormatTwSlotAtRule {
    fn fmt_fields(&self, node: &TwSlotAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwSlotAtRuleFields {
            slot_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&slot_token?).lowercase(),
                semicolon_token.format()
            ]
        )
    }
}
