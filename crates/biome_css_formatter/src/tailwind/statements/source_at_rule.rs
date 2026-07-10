use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{TwSourceAtRule, TwSourceAtRuleFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwSourceAtRule;
impl FormatNodeRule<TwSourceAtRule> for FormatTwSourceAtRule {
    fn fmt_fields(&self, node: &TwSourceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwSourceAtRuleFields {
            source_token,
            not_token,
            source,
            semicolon_token,
        } = node.as_fields();

        write!(f, [format_css_token(&source_token?).lowercase(), space()])?;
        if let Some(not_token) = not_token {
            write!(f, [format_css_token(&not_token).lowercase(), space()])?;
        }
        write!(f, [source.format(), semicolon_token.format()])
    }
}
