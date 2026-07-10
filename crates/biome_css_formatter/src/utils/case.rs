use crate::prelude::*;
use biome_css_syntax::{
    AnyCssQueryFeatureName, AnyCssSelectorIdentifier, CssContainerScrollStateQueryInParens,
    CssGenericComponentValueList, CssIdentifier, CssIfMediaTest, CssQualifiedRule, CssSyntaxNode,
    CssSyntaxToken,
};

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

/// Returns the case policy for CSS-wide keywords such as `INITIAL`.
pub(crate) fn css_wide_keyword_case(value: &CssIdentifier) -> CssCase {
    if is_lowercase_value_keyword(value) {
        CssCase::Lowercase
    } else {
        CssCase::Preserve
    }
}

/// Classifies an identifier formatted as an erased range-formatting root.
pub(crate) fn root_identifier_case(identifier: &CssIdentifier) -> Option<CssCase> {
    identifier
        .parent::<CssGenericComponentValueList>()
        .map(|_| css_wide_keyword_case(identifier))
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

pub(crate) fn query_feature_name_case(node: &AnyCssQueryFeatureName) -> CssCase {
    if node
        .as_css_identifier()
        .is_none_or(should_preserve_query_feature_identifier)
    {
        CssCase::Preserve
    } else {
        CssCase::Lowercase
    }
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

/// Returns the case policy for pseudo names such as `:--STATE`.
pub(crate) fn pseudo_name_case(name: &AnyCssSelectorIdentifier) -> CssCase {
    let Some(name) = name.as_css_identifier() else {
        return CssCase::Preserve;
    };

    let preserve = name
        .value_token()
        .is_ok_and(|token| token.token_text_trimmed().starts_with("--"))
        || name
            .syntax()
            .ancestors()
            .find_map(CssQualifiedRule::cast)
            .is_some_and(|rule| rule.prelude().syntax().text_trimmed().starts_with("---"));

    if preserve {
        CssCase::Preserve
    } else {
        CssCase::Lowercase
    }
}
