use crate::prelude::*;
use crate::utils::case::format_css_identifier;
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

        write!(f, [format_css_identifier(&first).preserve()])?;

        let mut previous = first;
        for modifier in modifiers {
            let has_source_gap = previous.syntax().text_trimmed_range().end()
                < modifier.syntax().text_trimmed_range().start();
            let separator = format_once(move |f| {
                if has_source_gap {
                    space().fmt(f)?;
                }
                Ok(())
            });

            write!(f, [separator, format_css_identifier(&modifier).preserve()])?;
            previous = modifier;
        }

        Ok(())
    }
}
