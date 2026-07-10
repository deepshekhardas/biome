use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssContainerStyleAndQuery, CssContainerStyleAndQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleAndQuery;
impl FormatNodeRule<CssContainerStyleAndQuery> for FormatCssContainerStyleAndQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleAndQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerStyleAndQueryFields {
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
