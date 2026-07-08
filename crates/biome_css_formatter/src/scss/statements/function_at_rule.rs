use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{ScssFunctionAtRule, ScssFunctionAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssFunctionAtRule;

impl FormatNodeRule<ScssFunctionAtRule> for FormatScssFunctionAtRule {
    fn fmt_fields(&self, node: &ScssFunctionAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssFunctionAtRuleFields {
            function_token,
            name,
            parameters,
            block,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&function_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve(),
                parameters.format(),
                space(),
                block.format()
            ]
        )
    }
}
