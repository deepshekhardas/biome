use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssSupportsNotCondition, CssSupportsNotConditionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsNotCondition;
impl FormatNodeRule<CssSupportsNotCondition> for FormatCssSupportsNotCondition {
    fn fmt_fields(&self, node: &CssSupportsNotCondition, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSupportsNotConditionFields { not_token, query } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&not_token?).preserve(),
                space(),
                query.format()
            ]
        )
    }
}
