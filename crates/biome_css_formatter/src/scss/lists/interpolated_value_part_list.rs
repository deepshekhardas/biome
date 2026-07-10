use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::ScssInterpolatedValuePartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedValuePartList;
impl FormatRule<ScssInterpolatedValuePartList> for FormatScssInterpolatedValuePartList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssInterpolatedValuePartList, f: &mut CssFormatter) -> FormatResult<()> {
        for item in node {
            format_css_identifier(&item).preserve().fmt(f)?;
        }

        Ok(())
    }
}
