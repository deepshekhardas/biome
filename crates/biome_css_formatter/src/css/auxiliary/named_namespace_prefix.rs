use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{CssNamedNamespacePrefix, CssNamedNamespacePrefixFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNamedNamespacePrefix;
impl FormatNodeRule<CssNamedNamespacePrefix> for FormatCssNamedNamespacePrefix {
    fn fmt_fields(&self, node: &CssNamedNamespacePrefix, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNamedNamespacePrefixFields { name } = node.as_fields();
        let name = name?;

        write!(f, [format_css_identifier(&name).preserve()])
    }
}
