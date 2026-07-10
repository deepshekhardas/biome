use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssContainerAndQuery, CssContainerAndQueryFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerAndQuery;
impl FormatNodeRule<CssContainerAndQuery> for FormatCssContainerAndQuery {
    fn fmt_fields(&self, node: &CssContainerAndQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssContainerAndQueryFields {
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
