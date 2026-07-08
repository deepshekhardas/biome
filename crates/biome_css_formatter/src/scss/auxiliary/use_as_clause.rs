use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{ScssUseAsClause, ScssUseAsClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssUseAsClause;

impl FormatNodeRule<ScssUseAsClause> for FormatScssUseAsClause {
    fn fmt_fields(&self, node: &ScssUseAsClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssUseAsClauseFields {
            as_token,
            namespace,
        } = node.as_fields();
        let namespace = namespace?;

        write!(
            f,
            [
                format_css_token(&as_token?).preserve(),
                space(),
                format_css_identifier(&namespace).preserve()
            ]
        )
    }
}
