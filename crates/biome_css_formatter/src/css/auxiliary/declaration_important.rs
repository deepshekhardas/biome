use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssDeclarationImportant, CssDeclarationImportantFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationImportant;
impl FormatNodeRule<CssDeclarationImportant> for FormatCssDeclarationImportant {
    fn fmt_fields(&self, node: &CssDeclarationImportant, f: &mut CssFormatter) -> FormatResult<()> {
        let CssDeclarationImportantFields {
            excl_token,
            important_token,
        } = node.as_fields();

        write!(
            f,
            [
                excl_token.format(),
                format_css_token(&important_token?).lowercase()
            ]
        )
    }
}
