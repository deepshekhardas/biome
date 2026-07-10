use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{TwCustomVariantAtRule, TwCustomVariantAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwCustomVariantAtRule;
impl FormatNodeRule<TwCustomVariantAtRule> for FormatTwCustomVariantAtRule {
    fn fmt_fields(&self, node: &TwCustomVariantAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwCustomVariantAtRuleFields {
            custom_variant_token,
            name,
            selector,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&custom_variant_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve(),
                space(),
                selector.format()
            ]
        )
    }
}
