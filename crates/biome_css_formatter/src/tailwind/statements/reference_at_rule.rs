use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{TwReferenceAtRule, TwReferenceAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwReferenceAtRule;
impl FormatNodeRule<TwReferenceAtRule> for FormatTwReferenceAtRule {
    fn fmt_fields(&self, node: &TwReferenceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwReferenceAtRuleFields {
            reference_token,
            path,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&reference_token?).lowercase(),
                space(),
                path.format(),
                semicolon_token.format()
            ]
        )
    }
}
