use crate::prelude::*;
use crate::utils::case::{format_css_identifier, should_preserve_query_feature_name};
use biome_css_syntax::{CssQueryFeatureRange, CssQueryFeatureRangeFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureRange;
impl FormatNodeRule<CssQueryFeatureRange> for FormatCssQueryFeatureRange {
    fn fmt_fields(&self, node: &CssQueryFeatureRange, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQueryFeatureRangeFields {
            left,
            comparison,
            right,
        } = node.as_fields();
        let left = left?;
        let formatted_left = if should_preserve_query_feature_name(&left) {
            format_css_identifier(&left).preserve()
        } else {
            format_css_identifier(&left).lowercase()
        };

        write!(
            f,
            [
                formatted_left,
                space(),
                comparison.format(),
                space(),
                right.format()
            ]
        )
    }
}
