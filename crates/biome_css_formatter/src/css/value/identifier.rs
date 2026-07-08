use crate::prelude::*;
use crate::utils::case::CssIdentifierCase;
use biome_css_syntax::{CssIdentifier, CssIdentifierFields};
use biome_formatter::{FormatRuleWithOptions, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIdentifier {
    case: CssIdentifierCase,
}

impl FormatRuleWithOptions<CssIdentifier> for FormatCssIdentifier {
    type Options = CssIdentifierCase;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}

impl FormatNodeRule<CssIdentifier> for FormatCssIdentifier {
    fn fmt_fields(&self, node: &CssIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIdentifierFields { value_token } = node.as_fields();

        #[cfg(debug_assertions)]
        if self.case == CssIdentifierCase::Auto {
            crate::utils::case::record_auto_identifier(node, f);
        }

        if self.resolve_case() == CssIdentifierCase::Lowercase {
            write!(f, [value_token.format()?.with_lowercase()])
        } else {
            write!(f, [value_token.format()])
        }
    }
}

impl FormatCssIdentifier {
    fn resolve_case(&self) -> CssIdentifierCase {
        match self.case {
            CssIdentifierCase::Lowercase => CssIdentifierCase::Lowercase,
            CssIdentifierCase::Auto | CssIdentifierCase::Preserve | CssIdentifierCase::Ignore => {
                CssIdentifierCase::Preserve
            }
        }
    }
}
