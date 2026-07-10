use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::TwApplyClassList;
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwApplyClassList;
impl FormatRule<TwApplyClassList> for FormatTwApplyClassList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &TwApplyClassList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut classes = node.iter();

        let Some(first) = classes.next() else {
            return Ok(());
        };

        write!(f, [format_css_identifier(&first).preserve()])?;

        for class in classes {
            write!(f, [space(), format_css_identifier(&class).preserve()])?;
        }

        Ok(())
    }
}
