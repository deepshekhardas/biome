use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssSupportsAtRuleDeclarator, CssSupportsAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAtRuleDeclarator;
impl FormatNodeRule<CssSupportsAtRuleDeclarator> for FormatCssSupportsAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssSupportsAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssSupportsAtRuleDeclaratorFields {
            supports_token,
            condition,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&supports_token?).lowercase(),
                space(),
                group(&indent(&condition.format())),
            ]
        )
    }
}
