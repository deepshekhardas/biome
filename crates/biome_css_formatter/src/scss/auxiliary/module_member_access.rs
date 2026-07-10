use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{ScssModuleMemberAccess, ScssModuleMemberAccessFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssModuleMemberAccess;
impl FormatNodeRule<ScssModuleMemberAccess> for FormatScssModuleMemberAccess {
    fn fmt_fields(&self, node: &ScssModuleMemberAccess, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssModuleMemberAccessFields {
            module,
            dot_token,
            member,
        } = node.as_fields();
        let member = member?;

        write!(
            f,
            [
                format_css_identifier(&module?).preserve(),
                dot_token.format(),
                format_css_identifier(&member).preserve()
            ]
        )
    }
}
