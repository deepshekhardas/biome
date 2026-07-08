use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{ScssIncludeAtRule, ScssIncludeAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssIncludeAtRule;

impl FormatNodeRule<ScssIncludeAtRule> for FormatScssIncludeAtRule {
    fn fmt_fields(&self, node: &ScssIncludeAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssIncludeAtRuleFields {
            include_token,
            name,
            arguments,
            using_clause,
            block,
            semicolon_token,
        } = node.as_fields();
        let name = name?;

        write!(
            f,
            [
                format_css_token(&include_token?).lowercase(),
                space(),
                format_css_identifier(&name).preserve(),
                arguments.format()
            ]
        )?;

        if let Some(using_clause) = using_clause {
            write!(f, [space(), using_clause.format()])?;
        }

        if let Some(block) = block {
            write!(f, [space(), block.format()])?;

            if let Some(semicolon_token) = semicolon_token {
                write!(f, [format_removed(&semicolon_token)])?;
            }
        } else {
            if let Some(semicolon_token) = semicolon_token {
                write!(f, [semicolon_token.format()])?;
            } else {
                write!(f, [token(";")])?;
            }
        }

        Ok(())
    }
}
