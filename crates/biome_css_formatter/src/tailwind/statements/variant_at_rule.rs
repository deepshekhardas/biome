use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{TwVariantAtRule, TwVariantAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwVariantAtRule;
impl FormatNodeRule<TwVariantAtRule> for FormatTwVariantAtRule {
    fn fmt_fields(&self, node: &TwVariantAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwVariantAtRuleFields {
            variant_token,
            name,
            block,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&variant_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve(),
                space(),
                block.format()
            ]
        )
    }
}
