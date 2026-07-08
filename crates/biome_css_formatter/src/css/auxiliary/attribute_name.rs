use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{CssAttributeName, CssAttributeNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeName;
impl FormatNodeRule<CssAttributeName> for FormatCssAttributeName {
    fn fmt_fields(&self, node: &CssAttributeName, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttributeNameFields { namespace, name } = node.as_fields();
        let name = name?;

        write!(
            f,
            [namespace.format(), format_css_identifier(&name).preserve()]
        )
    }
}
