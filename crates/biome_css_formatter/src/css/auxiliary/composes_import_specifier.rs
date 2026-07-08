use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{CssComposesImportSpecifier, CssComposesImportSpecifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComposesImportSpecifier;
impl FormatNodeRule<CssComposesImportSpecifier> for FormatCssComposesImportSpecifier {
    fn fmt_fields(
        &self,
        node: &CssComposesImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssComposesImportSpecifierFields { from_token, source } = node.as_fields();

        let source = source?;
        write![
            f,
            [
                space(),
                format_css_token(&from_token?).lowercase(),
                space(),
                format_css_identifier(&source).preserve()
            ]
        ]
    }
}
