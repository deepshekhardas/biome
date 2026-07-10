use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{ScssVariable, ScssVariableFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssVariable;

impl FormatNodeRule<ScssVariable> for FormatScssVariable {
    fn fmt_fields(&self, node: &ScssVariable, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssVariableFields { dollar_token, name } = node.as_fields();

        write!(
            f,
            [
                dollar_token.format(),
                format_css_identifier(&name?).preserve()
            ]
        )
    }
}
