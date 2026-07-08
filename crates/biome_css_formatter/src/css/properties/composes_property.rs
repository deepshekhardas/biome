use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{CssComposesProperty, CssComposesPropertyFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComposesProperty;
impl FormatNodeRule<CssComposesProperty> for FormatCssComposesProperty {
    fn fmt_fields(&self, node: &CssComposesProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssComposesPropertyFields {
            name,
            colon_token,
            values,
        } = node.as_fields();

        let name = name?;

        write!(
            f,
            [
                format_css_identifier(&name).lowercase(),
                colon_token.format()
            ]
        )?;

        if values.len() > 1 {
            return write!(
                f,
                [indent(&format_args![hard_line_break(), values.format()])]
            );
        }

        write!(f, [space(), values.format()])
    }
}
