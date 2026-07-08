use biome_css_syntax::{CssFunctionParameterDefaultValue, CssFunctionParameterDefaultValueFields};

use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionParameterDefaultValue;

impl FormatNodeRule<CssFunctionParameterDefaultValue> for FormatCssFunctionParameterDefaultValue {
    fn fmt_fields(
        &self,
        node: &CssFunctionParameterDefaultValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFunctionParameterDefaultValueFields { colon_token, value } = node.as_fields();

        write!(
            f,
            [
                colon_token.format(),
                space(),
                format_css_identifier(&value?).preserve()
            ]
        )
    }
}
