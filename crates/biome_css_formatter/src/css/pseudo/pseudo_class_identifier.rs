use crate::prelude::*;
use crate::utils::case::{format_css_identifier, should_preserve_pseudo_name};
use biome_css_syntax::{CssPseudoClassIdentifier, CssPseudoClassIdentifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassIdentifier;
impl FormatNodeRule<CssPseudoClassIdentifier> for FormatCssPseudoClassIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassIdentifierFields { name } = node.as_fields();
        let name = name?;

        if should_preserve_pseudo_name(&name) {
            write!(f, [format_css_identifier(&name).preserve()])
        } else {
            write!(f, [format_css_identifier(&name).lowercase()])
        }
    }
}
