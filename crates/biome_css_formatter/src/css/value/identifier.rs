use crate::prelude::*;
use biome_css_syntax::{CssIdentifier, CssIdentifierFields};
use biome_formatter::{FormatRuleWithOptions, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIdentifier {
    case: CssCase,
}

impl FormatRuleWithOptions<CssIdentifier> for FormatCssIdentifier {
    type Options = CssCase;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.case = options;
        self
    }
}

impl FormatNodeRule<CssIdentifier> for FormatCssIdentifier {
    fn fmt_fields(&self, node: &CssIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIdentifierFields { value_token } = node.as_fields();

        #[cfg(debug_assertions)]
        if self.case == CssCase::Auto {
            crate::utils::case::record_auto_identifier(node, f);
        }

        write!(f, [value_token.format().with_case(self.case)])
    }
}
