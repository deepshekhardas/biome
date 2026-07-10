use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{CssPageSelectorPseudo, CssPageSelectorPseudoFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageSelectorPseudo;
impl FormatNodeRule<CssPageSelectorPseudo> for FormatCssPageSelectorPseudo {
    fn fmt_fields(&self, node: &CssPageSelectorPseudo, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPageSelectorPseudoFields {
            colon_token,
            selector,
        } = node.as_fields();
        let selector = selector?;

        write!(
            f,
            [
                colon_token.format(),
                format_css_identifier(&selector).preserve()
            ]
        )
    }
}
