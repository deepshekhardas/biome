use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssRawStringDeclarator, CssRawStringDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRawStringDeclarator;

impl FormatNodeRule<CssRawStringDeclarator> for FormatCssRawStringDeclarator {
    fn fmt_fields(&self, node: &CssRawStringDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRawStringDeclaratorFields { raw_string_token } = node.as_fields();
        write!(f, [format_css_token(&raw_string_token?).lowercase()])
    }
}
