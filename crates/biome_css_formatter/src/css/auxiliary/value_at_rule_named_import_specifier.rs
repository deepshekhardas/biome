use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{
    CssValueAtRuleNamedImportSpecifier, CssValueAtRuleNamedImportSpecifierFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleNamedImportSpecifier;
impl FormatNodeRule<CssValueAtRuleNamedImportSpecifier>
    for FormatCssValueAtRuleNamedImportSpecifier
{
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleNamedImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssValueAtRuleNamedImportSpecifierFields {
            name,
            as_token,
            local_name,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_identifier(&name?).preserve(),
                space(),
                format_css_token(&as_token?).lowercase(),
                space(),
                format_css_identifier(&local_name?).preserve()
            ]
        )
    }
}
