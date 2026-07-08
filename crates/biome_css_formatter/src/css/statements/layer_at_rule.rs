use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssLayerAtRule, CssLayerAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLayerAtRule;
impl FormatNodeRule<CssLayerAtRule> for FormatCssLayerAtRule {
    fn fmt_fields(&self, node: &CssLayerAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssLayerAtRuleFields { layer_token, layer } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&layer_token?).lowercase(),
                space(),
                layer.format()
            ]
        )
    }
}
