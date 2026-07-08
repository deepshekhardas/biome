use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{CssMediaType, CssMediaTypeFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaType;
impl FormatNodeRule<CssMediaType> for FormatCssMediaType {
    fn fmt_fields(&self, node: &CssMediaType, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaTypeFields { value } = node.as_fields();
        let value = value?;

        write!(f, [format_css_identifier(&value).preserve()])
    }
}
