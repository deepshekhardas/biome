use biome_css_syntax::{CssReturnsStatement, CssReturnsStatementFields};
use biome_formatter::write;

use crate::prelude::*;
use crate::utils::case::format_css_token;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssReturnsStatement;

impl FormatNodeRule<CssReturnsStatement> for FormatCssReturnsStatement {
    fn fmt_fields(&self, node: &CssReturnsStatement, f: &mut CssFormatter) -> FormatResult<()> {
        let CssReturnsStatementFields { ty, returns_token } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&returns_token?).lowercase(),
                space(),
                ty.format(),
            ]
        )
    }
}
