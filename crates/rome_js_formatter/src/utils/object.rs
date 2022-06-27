use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::utils::StringLiteralParentKind;
use rome_formatter::write;
use rome_js_syntax::JsAnyObjectMemberName;
use rome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL;
use rome_rowan::AstNode;
use unicode_width::UnicodeWidthStr;

pub(crate) fn write_member_name(
    name: &JsAnyObjectMemberName,
    f: &mut JsFormatter,
) -> FormatResult<usize> {
    match name {
        name @ JsAnyObjectMemberName::JsLiteralMemberName(literal) => {
            let value = literal.value()?;

            if value.kind() == JS_STRING_LITERAL {
                let format = FormatLiteralStringToken::new(&value, StringLiteralParentKind::Member);
                let cleaned = format.clean_text(f.context());

                write!(f, [cleaned])?;

                Ok(cleaned.width())
            } else {
                write!(f, [name.format()])?;

                Ok(value.text_trimmed().width())
            }
        }
        name => {
            write!(f, [&name.format()])?;
            Ok(name.text().width())
        }
    }
}