use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssIfSassTest, CssIfSassTestFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfSassTest;
impl FormatNodeRule<CssIfSassTest> for FormatCssIfSassTest {
    fn fmt_fields(&self, node: &CssIfSassTest, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfSassTestFields {
            sass_token,
            l_paren_token,
            test,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_css_token(&sass_token?).lowercase(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&test.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
