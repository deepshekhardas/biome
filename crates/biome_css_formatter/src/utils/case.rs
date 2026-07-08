use crate::prelude::*;
use crate::{CssTokenCase, FormatCssSyntaxToken};
use biome_css_syntax::{
    AnyCssQueryFeatureName, CssContainerScrollStateQueryInParens, CssGenericProperty,
    CssIdentifier, CssIfMediaTest, CssLanguage, CssSyntaxNode, CssSyntaxToken,
    ScssInterpolatedIdentifier,
};
use biome_formatter::{Format, FormatRefWithRule};

/// Case policy for CSS identifiers such as `COLOR`.
///
/// Choose an explicit policy at formatter callsites. Raw `identifier.format()`
/// defaults to [`CssIdentifierCase::Auto`]: release builds preserve source text,
/// while debug builds report it as unclassified at formatter completion. This
/// keeps new syntax safe at runtime and makes formatter tests choose
/// [`CssIdentifierCase::Preserve`] or [`CssIdentifierCase::Lowercase`].
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(crate) enum CssIdentifierCase {
    /// No callsite policy was chosen.
    #[default]
    Auto,
    /// Preserve source casing.
    Preserve,
    /// Format source text as lowercase.
    Lowercase,
    /// Explicitly skip casing policy. Runtime preserves.
    Ignore,
}

pub(crate) fn format_css_identifier<T>(node: &T) -> FormatCssIdentifierCase<'_, T>
where
    T: FormatCssIdentifierCaseNode + ?Sized,
{
    FormatCssIdentifierCase {
        node,
        case: CssIdentifierCase::Auto,
    }
}

pub(crate) trait FormatCssIdentifierCaseNode {
    fn fmt_with_case(&self, case: CssIdentifierCase, f: &mut CssFormatter) -> FormatResult<()>;
}

pub(crate) struct FormatCssIdentifierCase<'a, T: ?Sized> {
    node: &'a T,
    case: CssIdentifierCase,
}

impl<T: ?Sized> FormatCssIdentifierCase<'_, T> {
    pub(crate) fn preserve(mut self) -> Self {
        self.case = CssIdentifierCase::Preserve;
        self
    }

    pub(crate) fn lowercase(mut self) -> Self {
        self.case = CssIdentifierCase::Lowercase;
        self
    }

    #[expect(dead_code)]
    pub(crate) fn ignore(mut self) -> Self {
        self.case = CssIdentifierCase::Ignore;
        self
    }
}

impl<T> Format<CssFormatContext> for FormatCssIdentifierCase<'_, T>
where
    T: FormatCssIdentifierCaseNode + ?Sized,
{
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        self.node.fmt_with_case(self.case, f)
    }
}

impl<T> FormatCssIdentifierCaseNode for T
where
    T: biome_rowan::AstNode<Language = CssLanguage> + AsFormat<CssFormatContext>,
{
    fn fmt_with_case(&self, case: CssIdentifierCase, f: &mut CssFormatter) -> FormatResult<()> {
        if let Some(identifier) = CssIdentifier::cast(self.syntax().clone()) {
            identifier.format().with_options(case).fmt(f)
        } else {
            self.format().fmt(f)
        }
    }
}

pub(crate) fn format_css_token(token: &CssSyntaxToken) -> FormatCssTokenCase<'_> {
    FormatCssTokenCase {
        token,
        case: CssTokenCase::Auto,
    }
}

pub(crate) struct FormatCssTokenCase<'a> {
    token: &'a CssSyntaxToken,
    case: CssTokenCase,
}

impl FormatCssTokenCase<'_> {
    pub(crate) fn preserve(mut self) -> Self {
        self.case = CssTokenCase::Preserve;
        self
    }

    pub(crate) fn lowercase(mut self) -> Self {
        self.case = CssTokenCase::Lowercase;
        self
    }

    #[expect(dead_code)]
    pub(crate) fn ignore(mut self) -> Self {
        self.case = CssTokenCase::Ignore;
        self
    }
}

impl Format<CssFormatContext> for FormatCssTokenCase<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        FormatRefWithRule::new(self.token, FormatCssSyntaxToken::default())
            .with_options(self.case)
            .fmt(f)
    }
}

#[cfg(debug_assertions)]
pub(crate) fn record_auto_identifier(node: &CssIdentifier, f: &mut CssFormatter) {
    let parent = node.syntax().parent().map(|parent| parent.kind());
    f.state_mut().record_audit_event(std::format!(
        "CSS formatter used an unclassified case policy: identifier `{}` under {parent:?}",
        node.syntax().text_trimmed()
    ));
}

#[cfg(debug_assertions)]
pub(crate) fn record_auto_contextual_token(token: &CssSyntaxToken, f: &mut CssFormatter) {
    if !token.kind().is_contextual_keyword() {
        return;
    }

    let parent = token.parent().map(|parent| parent.kind());
    f.state_mut().record_audit_event(std::format!(
        "CSS formatter used an unclassified case policy: token `{}` ({:?}) under {parent:?}",
        token.text_trimmed(),
        token.kind()
    ));
}

/// Formats CSS-wide keywords such as `INITIAL` as `initial`.
pub(crate) fn format_css_wide_keyword_value(
    value: &CssIdentifier,
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| {
        if is_lowercase_value_keyword(value) {
            format_css_identifier(value).lowercase().fmt(f)
        } else {
            format_css_identifier(value).preserve().fmt(f)
        }
    })
}

fn is_lowercase_value_keyword(value: &CssIdentifier) -> bool {
    value.value_token().is_ok_and(|token| {
        let text = token.token_text_trimmed();

        text.eq_ignore_ascii_case("initial")
            || text.eq_ignore_ascii_case("inherit")
            || text.eq_ignore_ascii_case("unset")
            || text.eq_ignore_ascii_case("revert")
            || text.eq_ignore_ascii_case("revert-layer")
    })
}

pub(crate) fn should_preserve_query_feature_name(node: &AnyCssQueryFeatureName) -> bool {
    node.as_css_identifier()
        .is_none_or(should_preserve_query_feature_identifier)
}

fn should_preserve_query_feature_identifier(node: &CssIdentifier) -> bool {
    is_dashed_identifier(node) || is_preserved_query_feature_context(node.syntax())
}

fn is_preserved_query_feature_context(node: &CssSyntaxNode) -> bool {
    node.ancestors().any(|ancestor| {
        CssContainerScrollStateQueryInParens::can_cast(ancestor.kind())
            || CssIfMediaTest::can_cast(ancestor.kind())
    })
}

fn is_dashed_identifier(identifier: &CssIdentifier) -> bool {
    identifier
        .value_token()
        .is_ok_and(|token| token.token_text_trimmed().starts_with("--"))
}

/// Preserves units in interpolated declaration names such as:
///
/// ```scss
/// #{$size + 15PX}: value;
/// ```
pub(crate) fn should_preserve_interpolated_property_dimension_unit_case(
    node: &CssSyntaxNode,
) -> bool {
    let Some(identifier) = node
        .ancestors()
        .skip(1)
        .find_map(ScssInterpolatedIdentifier::cast)
    else {
        return false;
    };

    identifier
        .syntax()
        .parent()
        .is_some_and(|parent| CssGenericProperty::can_cast(parent.kind()))
}
