use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{ScssImportAtRule, ScssImportAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssImportAtRule;

impl FormatNodeRule<ScssImportAtRule> for FormatScssImportAtRule {
    fn fmt_fields(&self, node: &ScssImportAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssImportAtRuleFields {
            import_token,
            imports,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&import_token?).lowercase(),
                space(),
                group(&indent(&imports.format())),
                semicolon_token.format()
            ]
        )
    }
}
