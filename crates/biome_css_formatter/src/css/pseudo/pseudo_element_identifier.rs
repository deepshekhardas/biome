use crate::prelude::*;
use crate::utils::case::format_css_identifier;
use biome_css_syntax::{
    AnyCssSelectorIdentifier, CssPseudoElementIdentifier, CssPseudoElementIdentifierFields,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementIdentifier;
impl FormatNodeRule<CssPseudoElementIdentifier> for FormatCssPseudoElementIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoElementIdentifierFields { name } = node.as_fields();
        let name = name?;

        if should_preserve_pseudo_selector_name(&name) {
            write!(f, [format_css_identifier(&name).preserve()])
        } else {
            write!(f, [format_css_identifier(&name).lowercase()])
        }
    }
}

fn should_preserve_pseudo_selector_name(name: &AnyCssSelectorIdentifier) -> bool {
    let Some(name) = name.as_css_identifier() else {
        return true;
    };

    name.value_token()
        .is_ok_and(|token| token.token_text_trimmed().starts_with("--"))
        || name
            .syntax()
            .ancestors()
            .find_map(biome_css_syntax::CssQualifiedRule::cast)
            .is_some_and(|rule| rule.prelude().syntax().text_trimmed().starts_with("---"))
}
