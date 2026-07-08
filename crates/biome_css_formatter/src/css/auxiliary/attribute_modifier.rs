use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssAttributeModifier, CssAttributeModifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeModifier;
impl FormatNodeRule<CssAttributeModifier> for FormatCssAttributeModifier {
    fn fmt_fields(&self, node: &CssAttributeModifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttributeModifierFields { value } = node.as_fields();

        write!(f, [format_css_token(&value?).lowercase()])
    }
}
