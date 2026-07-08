use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{CssTypeSelector, CssTypeSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTypeSelector;
impl FormatNodeRule<CssTypeSelector> for FormatCssTypeSelector {
    fn fmt_fields(&self, node: &CssTypeSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssTypeSelectorFields { namespace, ident } = node.as_fields();
        let ident = ident?;

        write!(
            f,
            [namespace.format(), format_css_identifier(&ident).preserve()]
        )
    }
}
