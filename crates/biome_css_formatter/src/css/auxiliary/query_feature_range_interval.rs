use crate::prelude::*;
use crate::utils::case::{format_css_identifier, should_preserve_query_feature_name};
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
        let formatted_name = if should_preserve_query_feature_name(&name) {
            format_css_identifier(&name).preserve()
        } else {
            format_css_identifier(&name).lowercase()
        };

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
