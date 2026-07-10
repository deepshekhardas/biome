use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssContainerOrQuery, CssContainerOrQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerOrQuery;
impl FormatNodeRule<CssContainerOrQuery> for FormatCssContainerOrQuery {
    fn fmt_fields(&self, node: &CssContainerOrQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssContainerOrQueryFields {
            left,
            or_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                format_css_token(&or_token?).preserve(),
                space(),
                right.format()
            ]
        )
    }
}
