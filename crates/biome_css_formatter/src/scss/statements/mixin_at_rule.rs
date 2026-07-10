use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{ScssMixinAtRule, ScssMixinAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMixinAtRule;

impl FormatNodeRule<ScssMixinAtRule> for FormatScssMixinAtRule {
    fn fmt_fields(&self, node: &ScssMixinAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMixinAtRuleFields {
            mixin_token,
            name,
            parameters,
            block,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&mixin_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve(),
                parameters.format(),
                space(),
                block.format()
            ]
        )
    }
}
