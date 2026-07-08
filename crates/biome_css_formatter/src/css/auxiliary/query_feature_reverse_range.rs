use crate::prelude::*;
use crate::utils::case::{format_css_identifier, should_preserve_query_feature_name};
use biome_css_syntax::{CssQueryFeatureReverseRange, CssQueryFeatureReverseRangeFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureReverseRange;
impl FormatNodeRule<CssQueryFeatureReverseRange> for FormatCssQueryFeatureReverseRange {
    fn fmt_fields(
        &self,
        node: &CssQueryFeatureReverseRange,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssQueryFeatureReverseRangeFields {
            left,
            comparison,
            right,
        } = node.as_fields();
        let right = right?;
        let formatted_right = if should_preserve_query_feature_name(&right) {
            format_css_identifier(&right).preserve()
        } else {
            format_css_identifier(&right).lowercase()
        };

        write!(
            f,
            [
                left.format(),
                space(),
                comparison.format(),
                space(),
                formatted_right
            ]
        )
    }
}
