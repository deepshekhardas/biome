use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssIfTestBooleanNotExpr, CssIfTestBooleanNotExprFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfTestBooleanNotExpr;

impl FormatNodeRule<CssIfTestBooleanNotExpr> for FormatCssIfTestBooleanNotExpr {
    fn fmt_fields(&self, node: &CssIfTestBooleanNotExpr, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfTestBooleanNotExprFields {
            not_token,
            expression,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&not_token?).lowercase(),
                space(),
                expression.format()
            ]
        )
    }
}
