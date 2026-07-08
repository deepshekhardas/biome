use crate::prelude::*;
use crate::utils::case::{format_css_identifier, format_css_token};
use biome_css_syntax::{
    ScssBinaryExpression, ScssBinaryExpressionFields, is_in_scss_control_condition_sequence,
    is_in_scss_parenthesized_expression, is_scss_parenthesized_expression,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssBinaryExpression;

impl FormatNodeRule<ScssBinaryExpression> for FormatScssBinaryExpression {
    fn fmt_fields(&self, node: &ScssBinaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let left = node.left()?;
        let formatted_right = FormatScssBinaryRightSide::new(node);

        if is_in_scss_control_condition_sequence(node) {
            write!(
                f,
                [format_css_identifier(&left).preserve(), formatted_right]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    format_css_identifier(&left).preserve(),
                    formatted_right
                ])]
            )
        }
    }
}

/// Formats the operator and right side, indenting grouped line breaks.
///
/// ```scss
/// $x: "a" +
///   "b";
/// ```
struct FormatScssBinaryRightSide<'a> {
    node: &'a ScssBinaryExpression,
}

impl<'a> FormatScssBinaryRightSide<'a> {
    fn new(node: &'a ScssBinaryExpression) -> Self {
        Self { node }
    }

    fn should_indent(&self) -> bool {
        if !is_in_scss_control_condition_sequence(self.node) {
            return true;
        }

        let ScssBinaryExpressionFields { right, .. } = self.node.as_fields();

        is_in_scss_parenthesized_expression(self.node)
            || right.as_ref().is_ok_and(is_scss_parenthesized_expression)
    }
}

impl Format<CssFormatContext> for FormatScssBinaryRightSide<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssBinaryExpressionFields {
            operator, right, ..
        } = self.node.as_fields();

        write!(f, [space(), format_css_token(&operator?).preserve()])?;

        if self.should_indent() {
            let right = right?;
            write!(
                f,
                [indent(&format_args![
                    soft_line_break_or_space(),
                    format_css_identifier(&right).preserve()
                ])]
            )
        } else {
            let right = right?;
            write!(
                f,
                [
                    soft_line_break_or_space(),
                    format_css_identifier(&right).preserve()
                ]
            )
        }
    }
}
