use crate::utils::case::format_css_identifier;
use crate::{FormatCssSyntaxToken, prelude::*};
use biome_css_syntax::{AnyCssPseudoValue, CssLanguage, CssPseudoValueList};
use biome_formatter::{trivia::FormatToken, write};
use biome_rowan::AstSeparatedElement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoValueList;
impl FormatRule<CssPseudoValueList> for FormatCssPseudoValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssPseudoValueList, f: &mut CssFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);
        let separator = soft_line_break_or_space();

        f.join_with(&separator)
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, element)| FormatPseudoValueItem {
                        last: index == last_index,
                        element,
                    }),
            )
            .finish()
    }
}

struct FormatPseudoValueItem {
    last: bool,
    element: AstSeparatedElement<CssLanguage, AnyCssPseudoValue>,
}

impl Format<CssFormatContext> for FormatPseudoValueItem {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = self.element.trailing_separator()?;
        let node = self.element.node()?;

        if let Some(identifier) = node.as_css_identifier() {
            format_css_identifier(identifier).preserve().fmt(f)?;
        } else {
            node.format().fmt(f)?;
        }

        if let Some(token) = separator {
            if self.last {
                FormatCssSyntaxToken::default().format_removed(token, f)?;
            } else {
                write!(f, [token.format()])?;
            }
        }

        Ok(())
    }
}
