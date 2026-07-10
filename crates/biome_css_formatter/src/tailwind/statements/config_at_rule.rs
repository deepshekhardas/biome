use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{TwConfigAtRule, TwConfigAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwConfigAtRule;
impl FormatNodeRule<TwConfigAtRule> for FormatTwConfigAtRule {
    fn fmt_fields(&self, node: &TwConfigAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwConfigAtRuleFields {
            config_token,
            path,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&config_token?).lowercase(),
                space(),
                path.format(),
                semicolon_token.format()
            ]
        )
    }
}
