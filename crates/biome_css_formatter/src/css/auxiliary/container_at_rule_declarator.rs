use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{CssContainerAtRuleDeclarator, CssContainerAtRuleDeclaratorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerAtRuleDeclarator;

impl FormatNodeRule<CssContainerAtRuleDeclarator> for FormatCssContainerAtRuleDeclarator {
    fn fmt_fields(
        &self,
        node: &CssContainerAtRuleDeclarator,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerAtRuleDeclaratorFields {
            container_token,
            name,
            query,
        } = node.as_fields();

        write!(
            f,
            [format_css_token(&container_token?).lowercase(), space()]
        )?;

        if let Some(name) = name {
            write!(f, [format_css_identifier(&name).preserve(), space()])?;
        }

        write!(f, [query.format()])
    }
}
