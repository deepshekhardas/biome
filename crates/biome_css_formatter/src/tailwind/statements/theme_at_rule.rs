use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{TwThemeAtRule, TwThemeAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwThemeAtRule;
impl FormatNodeRule<TwThemeAtRule> for FormatTwThemeAtRule {
    fn fmt_fields(&self, node: &TwThemeAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwThemeAtRuleFields {
            theme_token,
            name,
            block,
        } = node.as_fields();

        write!(f, [format_css_token(&theme_token?).lowercase()])?;
        if let Some(name) = name {
            write!(f, [space(), format_css_identifier(&name).preserve()])?;
        }
        write!(f, [space(), block.format()])
    }
}
