use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssKeyframesRangeSelector, CssKeyframesRangeSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesRangeSelector;
impl FormatNodeRule<CssKeyframesRangeSelector> for FormatCssKeyframesRangeSelector {
    fn fmt_fields(
        &self,
        node: &CssKeyframesRangeSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssKeyframesRangeSelectorFields { name, percentage } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&name?).preserve(),
                space(),
                percentage.format()
            ]
        )
    }
}
