use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssIfTestBooleanOrExpr, CssIfTestBooleanOrExprFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfTestBooleanOrExpr;

impl FormatNodeRule<CssIfTestBooleanOrExpr> for FormatCssIfTestBooleanOrExpr {
    fn fmt_fields(&self, node: &CssIfTestBooleanOrExpr, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfTestBooleanOrExprFields {
            left,
            or_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                format_css_token(&or_token?).lowercase(),
                space(),
                right.format()
            ]
        )
    }
}
