use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssSupportsAndCondition, CssSupportsAndConditionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAndCondition;
impl FormatNodeRule<CssSupportsAndCondition> for FormatCssSupportsAndCondition {
    fn fmt_fields(&self, node: &CssSupportsAndCondition, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSupportsAndConditionFields {
            left,
            and_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                format_css_token(&and_token?).preserve(),
                soft_line_break_or_space(),
                right.format()
            ]
        )
    }
}
