use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{
    AnyCssIfSupportsTestCondition, AnyCssImportSupportsCondition, AnyCssSupportsCondition,
    AnyCssSupportsInParens, CssIfSupportsTest, CssIfSupportsTestFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfSupportsTest;

impl FormatNodeRule<CssIfSupportsTest> for FormatCssIfSupportsTest {
    fn fmt_fields(&self, node: &CssIfSupportsTest, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfSupportsTestFields {
            supports_token,
            l_paren_token,
            test,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();
        let test = test?;

        write!(
            f,
            [
                format_css_token(&supports_token?).lowercase(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(
                        &format_if_supports_test_condition(&test),
                        should_insert_space
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }
}

fn format_if_supports_test_condition(
    test: &AnyCssIfSupportsTestCondition,
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| match test {
        AnyCssIfSupportsTestCondition::AnyCssImportSupportsCondition(condition) => {
            format_import_supports_condition(condition).fmt(f)
        }
        AnyCssIfSupportsTestCondition::CssIfSupportsIdentifierTest(test) => test.format().fmt(f),
    })
}

fn format_import_supports_condition(
    condition: &AnyCssImportSupportsCondition,
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| match condition {
        AnyCssImportSupportsCondition::AnyCssSupportsCondition(condition) => {
            format_supports_condition(condition).fmt(f)
        }
        AnyCssImportSupportsCondition::CssDeclaration(declaration) => declaration.format().fmt(f),
    })
}

fn format_supports_condition(
    condition: &AnyCssSupportsCondition,
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| match condition {
        AnyCssSupportsCondition::AnyCssSupportsInParens(condition) => {
            format_supports_in_parens(condition).fmt(f)
        }
        _ => condition.format().fmt(f),
    })
}

fn format_supports_in_parens(
    condition: &AnyCssSupportsInParens,
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| match condition {
        AnyCssSupportsInParens::AnyCssValue(value) => {
            format_css_identifier(value).preserve().fmt(f)
        }
        _ => condition.format().fmt(f),
    })
}
