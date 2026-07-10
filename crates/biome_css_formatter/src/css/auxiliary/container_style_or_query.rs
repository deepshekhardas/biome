use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssContainerStyleOrQuery, CssContainerStyleOrQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleOrQuery;
impl FormatNodeRule<CssContainerStyleOrQuery> for FormatCssContainerStyleOrQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleOrQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerStyleOrQueryFields {
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
