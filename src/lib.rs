//! This module contains utilities for parsing or validating strings relating
//! to tags.

use lazy_static::lazy_static;

// Source: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
static CONTEMPORARY_HTML_TAGS: [&str; 108] = [
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "hr",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "main",
    "main",
    "map",
    "mark",
    "menu",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "slot",
    "small",
    "source",
    "span",
    "strong",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "tr",
    "track",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];

// Source: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
static DEPRECATED_HTML_TAGS: [&str; 31] = [
    "acronym",
    "applet",
    "basefont",
    "bgsound",
    "big",
    "blink",
    "center",
    "command",
    "content",
    "dir",
    "element",
    "font",
    "frame",
    "frameset",
    "image",
    "isindex",
    "keygen",
    "listing",
    "marquee",
    "menuitem",
    "multicol",
    "nextid",
    "nobr",
    "noembed",
    "noframes",
    "plaintext",
    "shadow",
    "spacer",
    "strike",
    "tt",
    "xmp",
];

// Source: https://developer.mozilla.org/en-US/docs/Web/MathML/Element
static MATHML_TAGS: [&str; 44] = [
    "annotation-xml",
    "annotation",
    "maction",
    "maligngroup",
    "malignmark",
    "math",
    "menclose",
    "merror",
    "mfenced",
    "mfrac",
    "mglyph",
    "mi",
    "mlabeledtr",
    "mlongdiv",
    "mmultiscripts",
    "mn",
    "mo",
    "mover",
    "mpadded",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mscarries",
    "mscarry",
    "msgroup",
    "msline",
    "mspace",
    "msqrt",
    "msrow",
    "mstack",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "munder",
    "munderover",
    "none",
    "semantics",
];

// Source: https://developer.mozilla.org/en-US/docs/Web/SVG/Element
static SVG_TAGS: [&str; 90] = [
    "a",
    "altGlyph",
    "altGlyphDef",
    "altGlyphItem",
    "animate",
    "animateColor",
    "animateMotion",
    "animateTransform",
    "circle",
    "clipPath",
    "color-profile",
    "cursor",
    "defs",
    "desc",
    "discard",
    "ellipse",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feDropShadow",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "filter",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "font-face",
    "font",
    "foreignObject",
    "g",
    "glyph",
    "glyphRef",
    "hatch",
    "hatchpath",
    "hkern",
    "image",
    "line",
    "linearGradient",
    "marker",
    "mask",
    "mesh",
    "meshgradient",
    "meshpatch",
    "meshrow",
    "metadata",
    "missing-glyph",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "script",
    "set",
    "solidcolor",
    "stop",
    "style",
    "svg",
    "switch",
    "symbol",
    "text",
    "textPath",
    "title",
    "tref",
    "tspan",
    "unknown",
    "use",
    "view",
    "vkern",
];

lazy_static! {
    static ref DISALLOWED_CUSTOM_ELEMENT_TAGS: Vec<&'static str> = SVG_TAGS
        .iter()
        .chain(MATHML_TAGS.iter())
        .filter(|tag| tag.contains('-'))
        .map(|t| t.to_owned())
        .collect();
}

/// Returns true when the character provided is a "control" as defined
/// in [the WhatWG spec](https://infra.spec.whatwg.org/#control)
fn is_control(c: char) -> bool {
    match c {
        '\u{007F}'..='\u{009F}' => true,
        _ => is_c0_control(c),
    }
}

/// Returns true when the character provided is a "c0 control" as defined
/// in [the WhatWG spec](https://infra.spec.whatwg.org/#c0-control)
fn is_c0_control(c: char) -> bool {
    match c {
        '\u{0000}'..='\u{001F}' => true,
        _ => false,
    }
}

/// Returns true when the string provided is a "noncharacter" as defined
/// in [the WhatWG spec](https://infra.spec.whatwg.org/#noncharacter)
fn is_noncharacter(c: char) -> bool {
    match c {
        '\u{FDD0}'..='\u{FDEF}' => true,
        '\u{FFFE}' | '\u{FFFF}' | '\u{1FFFE}' | '\u{1FFFF}' | '\u{2FFFE}' | '\u{2FFFF}'
        | '\u{3FFFE}' | '\u{3FFFF}' | '\u{4FFFE}' | '\u{4FFFF}' | '\u{5FFFE}' | '\u{5FFFF}'
        | '\u{6FFFE}' | '\u{6FFFF}' | '\u{7FFFE}' | '\u{7FFFF}' | '\u{8FFFE}' | '\u{8FFFF}'
        | '\u{9FFFE}' | '\u{9FFFF}' | '\u{AFFFE}' | '\u{AFFFF}' | '\u{BFFFE}' | '\u{BFFFF}'
        | '\u{CFFFE}' | '\u{CFFFF}' | '\u{DFFFE}' | '\u{DFFFF}' | '\u{EFFFE}' | '\u{EFFFF}'
        | '\u{FFFFE}' | '\u{FFFFF}' | '\u{10FFFE}' | '\u{10FFFF}' => true,
        _ => false,
    }
}

/// Returns true when the string provided is a valid "attribute name" as defined
/// in [the WhatWG spec](https://html.spec.whatwg.org/multipage/syntax.html#syntax-attribute-name)
pub fn is_valid_html_attribute_name(attr: &str) -> bool {
    for c in attr.chars() {
        if is_noncharacter(c)
            || is_control(c)
            || c == '\u{0020}'
            || c == '\u{0022}'
            || c == '\u{0027}'
            || c == '\u{003E}'
            || c == '\u{002F}'
            || c == '\u{003D}'
        {
            return false;
        }
    }
    true
}

/// Returns true when the character provided is a valid PCENChar as defined
/// in [the WhatWG spec](https://html.spec.whatwg.org/multipage/custom-elements.html#prod-pcenchar)
fn is_pcen_char(c: char) -> bool {
    match c {
        '-' | '.' | '0'..='9' | 'a'..='z' | '_' => true,
        '\u{B7}' => true,
        '\u{C0}'..='\u{D6}' => true,
        '\u{D8}'..='\u{F6}' => true,
        '\u{F8}'..='\u{37D}' => true,
        '\u{37F}'..='\u{1FFF}' => true,
        '\u{200C}'..='\u{200D}' => true,
        '\u{203F}'..='\u{2040}' => true,
        '\u{2070}'..='\u{218F}' => true,
        '\u{2C00}'..='\u{2FEF}' => true,
        '\u{3001}'..='\u{D7FF}' => true,
        '\u{F900}'..='\u{FDCF}' => true,
        '\u{FDF0}'..='\u{FFFD}' => true,
        '\u{10000}'..='\u{EFFFF}' => true,
        _ => false,
    }
}

/// Returns true when the tag name provided would be a valid "custom element" per
/// [the WhatWG spec](https://html.spec.whatwg.org/multipage/custom-elements.html#valid-custom-element-name)
pub fn is_valid_custom_html_element_name(tag: &str) -> bool {
    if DISALLOWED_CUSTOM_ELEMENT_TAGS.contains(&tag) {
        return false;
    }

    let mut chars = tag.chars();
    let first_char = chars.next();

    match first_char {
        None => false,
        Some(first_char) => {
            // must begin with [a-z]
            if first_char < 'a' || first_char > 'z' {
                return false;
            }

            let mut seen_hyphen = false;
            for c in chars {
                if c == '-' {
                    seen_hyphen = true
                }

                // all characters must be valid PCENChar's
                if !is_pcen_char(c) {
                    return false;
                }
            }

            // must contain at least one hyphen
            seen_hyphen
        }
    }
}

/// Returns true when the tag name provided would be a valid HTML element
pub fn is_valid_html_element_name(tag: &str) -> bool {
    CONTEMPORARY_HTML_TAGS.contains(&tag) || DEPRECATED_HTML_TAGS.contains(&tag)
}

/// Returns true when the tag name provided would be a valid SVG element
pub fn is_valid_svg_element_name(tag: &str) -> bool {
    SVG_TAGS.contains(&tag)
}

/// Returns true when the tag name provided would be a valid MathML element
pub fn is_valid_mathml_element_name(tag: &str) -> bool {
    MATHML_TAGS.contains(&tag)
}

/// Returns true when you could validly construct a tag using this name in an HTML document
pub fn is_valid_sgml_tag(tag: &str) -> bool {
    is_valid_html_element_name(tag)
        || is_valid_svg_element_name(tag)
        || is_valid_mathml_element_name(tag)
        || is_valid_custom_html_element_name(tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_custom_element() {
        assert_eq!(is_valid_custom_html_element_name("foo-bar"), true);
        assert_eq!(is_valid_custom_html_element_name("foo-"), true);
        assert_eq!(is_valid_custom_html_element_name("bar-baz"), true);
    }

    #[test]
    fn invalid_custom_element() {
        assert_eq!(is_valid_custom_html_element_name("foobar"), false);
        assert_eq!(is_valid_custom_html_element_name("-bar"), false);
        assert_eq!(is_valid_custom_html_element_name("foo bar"), false);
        assert_eq!(is_valid_custom_html_element_name(""), false);
        assert_eq!(is_valid_custom_html_element_name("foo\nbar"), false);
        assert_eq!(is_valid_custom_html_element_name("-"), false);
    }

    #[test]
    fn valid_mathml_element() {
        assert_eq!(is_valid_mathml_element_name("annotation-xml"), true);
        assert_eq!(is_valid_mathml_element_name("munder"), true);
    }

    #[test]
    fn invalid_mathml_element() {
        assert_eq!(is_valid_mathml_element_name("svg"), false);
        assert_eq!(is_valid_mathml_element_name("b"), false);
    }
    
    #[test]
    fn valid_html_element() {
        assert_eq!(is_valid_html_element_name("section"), true);
        assert_eq!(is_valid_html_element_name("applet"), true);
    }
    
    #[test]
    fn invalid_html_element() {
        assert_eq!(is_valid_html_element_name("svg"), false);
        assert_eq!(is_valid_html_element_name("math"), false);
        assert_eq!(is_valid_html_element_name("circle"), false);
    }

    #[test]
    fn valid_svg_element() {
        assert_eq!(is_valid_svg_element_name("circle"), true);
        assert_eq!(is_valid_svg_element_name("g"), true);
    }

    #[test]
    fn invalid_svg_element() {
        assert_eq!(is_valid_svg_element_name("body"), false);
        assert_eq!(is_valid_svg_element_name("blockquote"), false);
    }

    #[test]
    fn valid_html_attribute() {
        assert_eq!(is_valid_html_attribute_name("-foo-bar"), true);
        assert_eq!(is_valid_html_attribute_name("data-foobar"), true);
    }

    #[test]
    fn invalid_html_attribute() {
        assert_eq!(is_valid_html_attribute_name("foo=bar"), false);
        assert_eq!(is_valid_html_attribute_name("\"foo\""), false);
        assert_eq!(is_valid_html_attribute_name("foo bar"), false);
    }
}