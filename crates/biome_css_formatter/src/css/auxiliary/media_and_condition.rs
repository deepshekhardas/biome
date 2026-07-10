use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssMediaAndCondition, CssMediaAndConditionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaAndCondition;
impl FormatNodeRule<CssMediaAndCondition> for FormatCssMediaAndCondition {
    fn fmt_fields(&self, node: &CssMediaAndCondition, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaAndConditionFields {
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
                space(),
                right.format()
            ]
        )
    }
}
