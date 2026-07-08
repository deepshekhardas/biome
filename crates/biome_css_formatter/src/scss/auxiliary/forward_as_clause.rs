use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{ScssForwardAsClause, ScssForwardAsClauseFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssForwardAsClause;

impl FormatNodeRule<ScssForwardAsClause> for FormatScssForwardAsClause {
    fn fmt_fields(&self, node: &ScssForwardAsClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssForwardAsClauseFields {
            as_token,
            prefix,
            star_token,
        } = node.as_fields();
        let prefix = prefix?;

        write!(
            f,
            [group(&format_args![
                format_css_token(&as_token?).preserve(),
                space(),
                format_css_identifier(&prefix).preserve(),
                star_token.format()
            ])]
        )
    }
}
