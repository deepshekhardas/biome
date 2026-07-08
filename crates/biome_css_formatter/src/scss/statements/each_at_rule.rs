use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{ScssEachAtRule, ScssEachAtRuleFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachAtRule;

impl FormatNodeRule<ScssEachAtRule> for FormatScssEachAtRule {
    fn fmt_fields(&self, node: &ScssEachAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssEachAtRuleFields {
            each_token,
            header,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&each_token?).lowercase(),
                group(&format_args![
                    space(),
                    indent(&group(&header.format())),
                    soft_line_break_or_space()
                ]),
                block.format()
            ]
        )
    }
}
