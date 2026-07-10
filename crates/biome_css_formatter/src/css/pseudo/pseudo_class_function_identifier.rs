use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{CssPseudoClassFunctionIdentifier, CssPseudoClassFunctionIdentifierFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionIdentifier;
impl FormatNodeRule<CssPseudoClassFunctionIdentifier> for FormatCssPseudoClassFunctionIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionIdentifierFields {
            name,
            l_paren_token,
            ident,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();
        let name = name?;
        let ident = ident?;

        write!(
            f,
            [
                format_css_identifier(&name).lowercase(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(
                        &format_css_identifier(&ident).preserve(),
                        should_insert_space
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
