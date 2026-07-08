use crate::prelude::*;
use crate::utils::case::{format_css_identifier, should_preserve_query_feature_name};
use biome_css_syntax::{CssQueryFeatureBoolean, CssQueryFeatureBooleanFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureBoolean;
impl FormatNodeRule<CssQueryFeatureBoolean> for FormatCssQueryFeatureBoolean {
    fn fmt_fields(&self, node: &CssQueryFeatureBoolean, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQueryFeatureBooleanFields { name } = node.as_fields();
        let name = name?;

        if should_preserve_query_feature_name(&name) {
            write!(f, [format_css_identifier(&name).preserve()])
        } else {
            write!(f, [format_css_identifier(&name).lowercase()])
        }
    }
}
