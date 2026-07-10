use crate::prelude::*;
use biome_css_syntax::CssUrlModifierList;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlModifierList;
impl FormatRule<CssUrlModifierList> for FormatCssUrlModifierList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssUrlModifierList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut modifiers = node.iter();

        let Some(first) = modifiers.next() else {
            return Ok(());
        };

        write!(f, [first.format().with_case(CssCase::Preserve)])?;

        let mut previous = first;
        for modifier in modifiers {
            let has_source_gap = previous.syntax().text_trimmed_range().end()
                < modifier.syntax().text_trimmed_range().start();
            write!(
                f,
                [
                    maybe_space(has_source_gap),
                    modifier.format().with_case(CssCase::Preserve)
                ]
            )?;
            previous = modifier;
        }

        Ok(())
    }
}
