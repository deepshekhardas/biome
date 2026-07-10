use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssStartingStyleAtRuleDeclarator, CssStartingStyleAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssStartingStyleAtRuleDeclarator;

impl FormatNodeRule<CssStartingStyleAtRuleDeclarator> for FormatCssStartingStyleAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssStartingStyleAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssStartingStyleAtRuleDeclaratorFields {
            starting_style_token,
        } = node.as_fields();

        write!(f, [format_css_token(&starting_style_token?).lowercase()])
    }
}
