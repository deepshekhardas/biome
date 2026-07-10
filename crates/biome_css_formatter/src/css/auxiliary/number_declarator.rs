use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssNumberDeclarator, CssNumberDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNumberDeclarator;

impl FormatNodeRule<CssNumberDeclarator> for FormatCssNumberDeclarator {
    fn fmt_fields(&self, node: &CssNumberDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNumberDeclaratorFields { number_token } = node.as_fields();
        write!(f, [format_css_token(&number_token?).lowercase()])
    }
}
