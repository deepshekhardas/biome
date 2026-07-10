use crate::{
    prelude::*,
    utils::string_utils::{FormatDimensionUnit, dimension_unit_case},
};
use biome_css_syntax::{CssRegularDimension, CssRegularDimensionFields};
use biome_formatter::{token::number::NumberFormatOptions, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularDimension;
impl FormatNodeRule<CssRegularDimension> for FormatCssRegularDimension {
    fn fmt_fields(&self, node: &CssRegularDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRegularDimensionFields {
            value_token,
            unit_token,
        } = node.as_fields();

        let unit =
            FormatDimensionUnit::from(unit_token?).with_case(dimension_unit_case(node.syntax()));

        write!(
            f,
            [
                format_number_token(&value_token?, NumberFormatOptions::default()),
                unit,
            ]
        )
    }
}
