use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{ScssForAtRule, ScssForAtRuleFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssForAtRule;

impl FormatNodeRule<ScssForAtRule> for FormatScssForAtRule {
    fn fmt_fields(&self, node: &ScssForAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssForAtRuleFields {
            for_token,
            variable,
            from_token,
            lower_bound,
            operator,
            upper_bound,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&for_token?).lowercase(),
                space(),
                group(&format_args![
                    variable.format(),
                    indent(&format_args![
                        soft_line_break_or_space(),
                        format_css_token(&from_token?).preserve(),
                        space(),
                        lower_bound.format(),
                        soft_line_break_or_space(),
                        format_css_token(&operator?).preserve(),
                        space(),
                        upper_bound.format()
                    ]),
                    soft_line_break_or_space()
                ]),
                block.format()
            ]
        )
    }
}
