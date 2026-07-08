use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssIfTestBooleanAndExpr, CssIfTestBooleanAndExprFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfTestBooleanAndExpr;

impl FormatNodeRule<CssIfTestBooleanAndExpr> for FormatCssIfTestBooleanAndExpr {
    fn fmt_fields(&self, node: &CssIfTestBooleanAndExpr, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfTestBooleanAndExprFields {
            left,
            and_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                format_css_token(&and_token?).lowercase(),
                space(),
                right.format()
            ]
        )
    }
}
