use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssContainerNotQuery, CssContainerNotQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerNotQuery;
impl FormatNodeRule<CssContainerNotQuery> for FormatCssContainerNotQuery {
    fn fmt_fields(&self, node: &CssContainerNotQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssContainerNotQueryFields { not_token, query } = node.as_fields();

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
