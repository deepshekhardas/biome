use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{CssNamespaceAtRule, CssNamespaceAtRuleFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNamespaceAtRule;
impl FormatNodeRule<CssNamespaceAtRule> for FormatCssNamespaceAtRule {
    fn fmt_fields(&self, node: &CssNamespaceAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNamespaceAtRuleFields {
            namespace_token,
            prefix,
            url,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&namespace_token?).lowercase(),
                space(),
                prefix
                    .as_ref()
                    .map(|prefix| format_css_identifier(prefix).preserve()),
                space(),
                url.format(),
                semicolon_token.format(),
            ]
        )
    }
}
