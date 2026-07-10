use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssContainerStyleNotQuery, CssContainerStyleNotQueryFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleNotQuery;
impl FormatNodeRule<CssContainerStyleNotQuery> for FormatCssContainerStyleNotQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleNotQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerStyleNotQueryFields { not_token, query } = node.as_fields();

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
