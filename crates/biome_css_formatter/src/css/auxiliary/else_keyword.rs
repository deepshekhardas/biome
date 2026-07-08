use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssElseKeyword, CssElseKeywordFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssElseKeyword;

impl FormatNodeRule<CssElseKeyword> for FormatCssElseKeyword {
    fn fmt_fields(&self, node: &CssElseKeyword, f: &mut CssFormatter) -> FormatResult<()> {
        let CssElseKeywordFields { else_token } = node.as_fields();

        write!(f, [format_css_token(&else_token?).lowercase()])
    }
}
