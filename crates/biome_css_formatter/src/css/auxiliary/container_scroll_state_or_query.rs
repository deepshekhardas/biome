use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssContainerScrollStateOrQuery, CssContainerScrollStateOrQueryFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerScrollStateOrQuery;
impl FormatNodeRule<CssContainerScrollStateOrQuery> for FormatCssContainerScrollStateOrQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerScrollStateOrQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerScrollStateOrQueryFields {
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
