use biome_css_syntax::{CssFunctionAtRuleDeclarator, CssFunctionAtRuleDeclaratorFields};
use biome_formatter::{format_args, write};

use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionAtRuleDeclarator;

impl FormatNodeRule<CssFunctionAtRuleDeclarator> for FormatCssFunctionAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssFunctionAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFunctionAtRuleDeclaratorFields {
            function_token,
            name,
            r_paren_token,
            parameters,
            l_paren_token,
            returns,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&function_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&parameters.format()),
                    r_paren_token.format()
                ]),
                space(),
                returns.format(),
            ]
        )
    }
}
