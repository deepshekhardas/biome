use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{TwApplyAtRule, TwApplyAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwApplyAtRule;
impl FormatNodeRule<TwApplyAtRule> for FormatTwApplyAtRule {
    fn fmt_fields(&self, node: &TwApplyAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwApplyAtRuleFields {
            apply_token,
            classes,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&apply_token?).lowercase(),
                space(),
                classes.format(),
                semicolon_token.format()
            ]
        )
    }
}
