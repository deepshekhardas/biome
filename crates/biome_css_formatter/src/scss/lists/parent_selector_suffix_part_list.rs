use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::ScssParentSelectorSuffixPartList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParentSelectorSuffixPartList;
impl FormatRule<ScssParentSelectorSuffixPartList> for FormatScssParentSelectorSuffixPartList {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &ScssParentSelectorSuffixPartList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        for item in node {
            format_css_identifier(&item).preserve().fmt(f)?;
        }

        Ok(())
    }
}
