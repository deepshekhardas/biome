use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::CssPseudoElementFunctionParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementFunctionParameterList;
impl FormatRule<CssPseudoElementFunctionParameterList>
    for FormatCssPseudoElementFunctionParameterList
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &CssPseudoElementFunctionParameterList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for parameter in node.iter() {
            joiner.entry(&format_css_identifier(&parameter).preserve());
        }

        joiner.finish()
    }
}
