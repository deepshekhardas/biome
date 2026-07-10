use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{ScssIncludeUsingClause, ScssIncludeUsingClauseFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssIncludeUsingClause;

impl FormatNodeRule<ScssIncludeUsingClause> for FormatScssIncludeUsingClause {
    fn fmt_fields(&self, node: &ScssIncludeUsingClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssIncludeUsingClauseFields {
            using_token,
            parameters,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&using_token?).preserve(),
                space(),
                parameters.format()
            ]
        )
    }
}
