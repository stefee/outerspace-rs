fn is_non_whitespace(c: char) -> bool {
    !c.is_whitespace()
}

fn format_wrap(
    s: &str,
    prefix: &str,
    suffix: &str,
    first_non_whitespace: Option<usize>,
    last_non_whitespace: Option<usize>,
) -> String {
    match (first_non_whitespace, last_non_whitespace) {
        (Some(start), Some(end)) => {
            let (leading_ws, rest) = s.split_at(start);
            let (rest, trailing_ws) = rest.split_at(end - start + 1);
            format!("{}{}{}{}{}", leading_ws, prefix, rest, suffix, trailing_ws)
        }
        (Some(start), None) => {
            let (leading_ws, rest) = s.split_at(start);
            format!("{}{}{}{}", leading_ws, prefix, rest, suffix)
        }
        (None, Some(end)) => {
            let (rest, trailing_ws) = s.split_at(end + 1);
            format!("{}{}{}{}", prefix, rest, suffix, trailing_ws)
        }
        (None, None) => {
            format!("{}{}{}", prefix, s, suffix)
        }
    }
}

pub fn wrap_non_whitespace(s: &str, prefix: &str, suffix: &str) -> String {
    let first_non_whitespace = s.find(is_non_whitespace);
    let last_non_whitespace = s.rfind(is_non_whitespace);
    format_wrap(s, prefix, suffix, first_non_whitespace, last_non_whitespace)
}

pub fn prefix_non_whitespace(s: &str, prefix: &str) -> String {
    let first_non_whitespace = s.find(is_non_whitespace);
    format_wrap(s, prefix, "", first_non_whitespace, None)
}

pub fn suffix_non_whitespace(s: &str, suffix: &str) -> String {
    let last_non_whitespace = s.rfind(is_non_whitespace);
    format_wrap(s, "", suffix, None, last_non_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_works() {
        assert_eq!(wrap_non_whitespace("", "<", ">"), "<>");
        assert_eq!(wrap_non_whitespace("  \n ", "<", ">"), "<  \n >");
        assert_eq!(wrap_non_whitespace("cosy", "<", ">"), "<cosy>");
        assert_eq!(
            wrap_non_whitespace("cosy matthew", "<", ">"),
            "<cosy matthew>"
        );
        assert_eq!(
            wrap_non_whitespace("\n \ncosy \nmatthew\n \n", "<", ">"),
            "\n \n<cosy \nmatthew>\n \n"
        );
    }

    #[test]
    fn prefix_works() {
        assert_eq!(prefix_non_whitespace("", "**"), "**");
        assert_eq!(prefix_non_whitespace("  \n ", "**"), "**  \n ");
        assert_eq!(prefix_non_whitespace("emboldened", "**"), "**emboldened");
        assert_eq!(
            prefix_non_whitespace("emboldened matthew", "**"),
            "**emboldened matthew"
        );
        assert_eq!(
            prefix_non_whitespace("\n \nemboldened \nmatthew", "**"),
            "\n \n**emboldened \nmatthew"
        );
    }

    #[test]
    fn suffix_works() {
        assert_eq!(suffix_non_whitespace("", "**"), "**");
        assert_eq!(suffix_non_whitespace("  \n ", "**"), "  \n **");
        assert_eq!(suffix_non_whitespace("emboldened", "**"), "emboldened**");
        assert_eq!(
            suffix_non_whitespace("emboldened matthew", "**"),
            "emboldened matthew**"
        );
        assert_eq!(
            suffix_non_whitespace("emboldened \nmatthew\n \n", "**"),
            "emboldened \nmatthew**\n \n"
        );
    }
}
