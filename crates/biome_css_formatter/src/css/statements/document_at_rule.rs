use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssDocumentAtRule, CssDocumentAtRuleFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDocumentAtRule;
impl FormatNodeRule<CssDocumentAtRule> for FormatCssDocumentAtRule {
    fn fmt_fields(&self, node: &CssDocumentAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssDocumentAtRuleFields {
            document_token,
            matchers,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&document_token?).lowercase(),
                space(),
                matchers.format(),
                space(),
                block.format()
            ]
        )
    }
}
