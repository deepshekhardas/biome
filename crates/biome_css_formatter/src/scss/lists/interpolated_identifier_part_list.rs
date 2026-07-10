use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::ScssInterpolatedIdentifierPartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedIdentifierPartList;
impl FormatRule<ScssInterpolatedIdentifierPartList> for FormatScssInterpolatedIdentifierPartList {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &ScssInterpolatedIdentifierPartList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        for item in node {
            format_css_identifier(&item).preserve().fmt(f)?;
        }

        Ok(())
    }
}
