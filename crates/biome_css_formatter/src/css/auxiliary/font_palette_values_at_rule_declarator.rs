use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{
    CssFontPaletteValuesAtRuleDeclarator, CssFontPaletteValuesAtRuleDeclaratorFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontPaletteValuesAtRuleDeclarator;

impl FormatNodeRule<CssFontPaletteValuesAtRuleDeclarator>
    for FormatCssFontPaletteValuesAtRuleDeclarator
{
    fn fmt_fields(
        &self,
        node: &CssFontPaletteValuesAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFontPaletteValuesAtRuleDeclaratorFields {
            font_palette_values_token,
            name,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&font_palette_values_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve()
            ]
        )
    }
}
