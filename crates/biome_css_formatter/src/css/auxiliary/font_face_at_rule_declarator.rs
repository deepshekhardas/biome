use crate::prelude::*;
use crate::utils::case::format_css_token;
use biome_css_syntax::{CssFontFaceAtRuleDeclarator, CssFontFaceAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFaceAtRuleDeclarator;

impl FormatNodeRule<CssFontFaceAtRuleDeclarator> for FormatCssFontFaceAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssFontFaceAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFontFaceAtRuleDeclaratorFields { font_face_token } = node.as_fields();

        write!(f, [format_css_token(&font_face_token?).lowercase()])
    }
}
