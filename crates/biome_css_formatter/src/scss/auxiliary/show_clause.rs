use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{ScssShowClause, ScssShowClauseFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssShowClause;

impl FormatNodeRule<ScssShowClause> for FormatScssShowClause {
    fn fmt_fields(&self, node: &ScssShowClause, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssShowClauseFields {
            show_token,
            members,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                format_css_token(&show_token?).preserve(),
                space(),
                members.format()
            ])]
        )
    }
}
