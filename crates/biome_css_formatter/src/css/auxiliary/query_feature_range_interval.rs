use crate::prelude::*;
use crate::utils::case::query_feature_name_case;
use biome_css_syntax::{CssQueryFeatureRangeInterval, CssQueryFeatureRangeIntervalFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureRangeInterval;
impl FormatNodeRule<CssQueryFeatureRangeInterval> for FormatCssQueryFeatureRangeInterval {
    fn fmt_fields(
        &self,
        node: &CssQueryFeatureRangeInterval,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssQueryFeatureRangeIntervalFields {
            left,
            left_comparison,
            name,
            right_comparison,
            right,
        } = node.as_fields();
        let name = name?;
        let formatted_name = name.format().with_case(query_feature_name_case(&name));

        write!(
            f,
            [
                left.format(),
                space(),
                left_comparison.format(),
                space(),
                formatted_name,
                space(),
                right_comparison.format(),
                space(),
                right.format()
            ]
        )
    }
}
