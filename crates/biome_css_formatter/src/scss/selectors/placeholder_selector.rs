use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{
    AnyCssSelectorCustomIdentifier, ScssPlaceholderSelector, ScssPlaceholderSelectorFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssPlaceholderSelector;
impl FormatNodeRule<ScssPlaceholderSelector> for FormatScssPlaceholderSelector {
    fn fmt_fields(&self, node: &ScssPlaceholderSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssPlaceholderSelectorFields {
            percent_token,
            name,
        } = node.as_fields();
        let name = name?;
        let formatted_name = format_with(|f| match &name {
            AnyCssSelectorCustomIdentifier::CssCustomIdentifier(name) => {
                format_css_identifier(name).preserve().fmt(f)
            }
            AnyCssSelectorCustomIdentifier::ScssInterpolatedIdentifier(name) => {
                name.format().fmt(f)
            }
        });

        write!(f, [percent_token.format(), formatted_name])
    }
}
