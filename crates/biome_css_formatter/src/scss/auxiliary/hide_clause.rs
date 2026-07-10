use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{ScssHideClause, ScssHideClauseFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssHideClause;

impl FormatNodeRule<ScssHideClause> for FormatScssHideClause {
    fn fmt_fields(&self, node: &ScssHideClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssHideClauseFields {
            hide_token,
            members,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                format_css_token(&hide_token?).preserve(),
                space(),
                members.format()
            ])]
        )
    }
}
