use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{CssValueAtRuleImportSpecifier, CssValueAtRuleImportSpecifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleImportSpecifier;
impl FormatNodeRule<CssValueAtRuleImportSpecifier> for FormatCssValueAtRuleImportSpecifier {
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssValueAtRuleImportSpecifierFields { name } = node.as_fields();
        let name = name?;

        write!(f, [format_css_identifier(&name).preserve()])
    }
}
