use crate::prelude::*;
use crate::utils::case::query_feature_name_case;
use biome_css_syntax::{CssQueryFeatureBoolean, CssQueryFeatureBooleanFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureBoolean;
impl FormatNodeRule<CssQueryFeatureBoolean> for FormatCssQueryFeatureBoolean {
    fn fmt_fields(&self, node: &CssQueryFeatureBoolean, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQueryFeatureBooleanFields { name } = node.as_fields();
        let name = name?;

        write!(f, [name.format().with_case(query_feature_name_case(&name))])
    }
}
