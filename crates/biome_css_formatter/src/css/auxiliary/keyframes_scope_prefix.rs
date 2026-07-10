use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssKeyframesScopePrefix, CssKeyframesScopePrefixFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesScopePrefix;
impl FormatNodeRule<CssKeyframesScopePrefix> for FormatCssKeyframesScopePrefix {
    fn fmt_fields(&self, node: &CssKeyframesScopePrefix, f: &mut CssFormatter) -> FormatResult<()> {
        let CssKeyframesScopePrefixFields { scope, name } = node.as_fields();

        write!(
            f,
            [format_css_token(&scope?).preserve(), space(), name.format(),]
        )
    }
}
