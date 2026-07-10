use crate::{
    prelude::*,
    utils::string_utils::{FormatDimensionUnit, dimension_unit_case},
};
use biome_css_syntax::{CssUnknownDimension, CssUnknownDimensionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownDimension;
impl FormatNodeRule<CssUnknownDimension> for FormatCssUnknownDimension {
    fn fmt_fields(&self, node: &CssUnknownDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnknownDimensionFields {
            value_token,
            unit_token,
        } = node.as_fields();

        let unit =
            FormatDimensionUnit::from(unit_token?).with_case(dimension_unit_case(node.syntax()));

        let var_name = write!(
            f,
            [value_token.format().with_case(CssCase::Lowercase), unit,]
        );
        var_name
    }
}
