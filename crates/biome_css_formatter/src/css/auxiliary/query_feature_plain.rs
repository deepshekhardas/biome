use crate::prelude::*;
use crate::utils::case::{format_css_identifier, should_preserve_query_feature_name};
use biome_css_syntax::{CssQueryFeaturePlain, CssQueryFeaturePlainFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeaturePlain;
impl FormatNodeRule<CssQueryFeaturePlain> for FormatCssQueryFeaturePlain {
    fn fmt_fields(&self, node: &CssQueryFeaturePlain, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQueryFeaturePlainFields {
            name,
            colon_token,
            value,
        } = node.as_fields();
        let name = name?;
        let value = value?;
        let formatted_name = if should_preserve_query_feature_name(&name) {
            format_css_identifier(&name).preserve()
        } else {
            format_css_identifier(&name).lowercase()
        };

        write!(
            f,
            [
                formatted_name,
                colon_token.format(),
                space(),
                format_css_identifier(&value).preserve()
            ]
        )
    }
}
