use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssMarginAtRule, CssMarginAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMarginAtRule;
impl FormatNodeRule<CssMarginAtRule> for FormatCssMarginAtRule {
    fn fmt_fields(&self, node: &CssMarginAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMarginAtRuleFields {
            at_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                at_token.format(),
                format_css_token(&name?).lowercase(),
                space(),
                block.format()
            ]
        )
    }
}
