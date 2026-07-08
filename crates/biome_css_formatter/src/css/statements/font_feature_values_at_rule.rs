use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssFontFeatureValuesAtRule, CssFontFeatureValuesAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesAtRule;

impl FormatNodeRule<CssFontFeatureValuesAtRule> for FormatCssFontFeatureValuesAtRule {
    fn fmt_fields(
        &self,
        node: &CssFontFeatureValuesAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFontFeatureValuesAtRuleFields {
            font_feature_values_token,
            names,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&font_feature_values_token?).lowercase(),
                space(),
                names.format(),
                space(),
                block.format()
            ]
        )
    }
}
