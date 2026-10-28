//! Spike: verify every upstream `re` pattern compiles under `fancy-regex`
//! with identical match semantics on representative inputs.
//!
//! Throwaway verification, not shipped logic (kept as a test so the
//! pattern set stays honest as consts land).

use fancy_regex::Regex as F;

fn check(pat: &str, text: &str, expect_some: bool) {
    let re = F::new(pat).unwrap_or_else(|e| panic!("compile fail {pat:?}: {e}"));
    let got = re.find(text).unwrap().is_some();
    assert_eq!(got, expect_some, "pattern {pat:?} on {text:?}");
}

#[test]
fn spike_all_upstream_patterns_compile() {
    // preprocessors
    check(r#"(?<=\n) +\n"#, "a\n   \nb", true);
    // htmlparser monkeypatched
    check(r#"<[a-zA-Z]|</>"#, "</>", true);
    check(r#"</[a-zA-Z]?"#, "</div", true);
    check(r#"\?>"#, "a?>b", true);
    check(r#"&([a-zA-Z][-.a-zA-Z0-9]*);"#, "&amp;", true);
    // blockprocessors
    check(r#"^(([ ]{4})+)"#, "        x", true);
    check(r#"(^|\n)[ ]{0,3}>[ ]?(.*)"#, "> quote", true);
    check(r#"^[ ]{0,3}\d+\.[ ]+(.*)"#, "1. item", true);
    check(r#"^[ ]{0,3}((\d+\.)|[*+-])[ ]+(.*)"#, "- item", true);
    check(r#"^[ ]{4,7}((\d+\.)|[*+-])[ ]+.*"#, "      - sub", true);
    check(
        r#"(?:^|\n)(?P<level>#{1,6})(?P<header>(?:\\.|[^\\])*?)#*(?:\n|$)"#,
        "# hi\n",
        true,
    );
    check(r#"^.*?\n(?:=+|-+)[ ]*(\n|$)"#, "Title\n=====\n", true);
    check(
        r#"^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$"#,
        "***",
        true,
    );
    check(
        r#"^[ ]{0,3}\[([^\[\]]*)\]:[ ]*(?:\n[ ]*)?([^\s]+)[ ]*(?:\n[ ]*)?((([\"'])(.*)\4[ ]*|\((.*)\)[ ]*)?$"#,
        "[id]: http://example.com \"title\"",
        true,
    );
    // inlinepatterns
    check(r#"(?<!\!)\["#, "[link]", true);
    check(r#"(?<!\!)\["#, "![img]", false);
    check(r#"(?:(?<!\\)((?:\\{2})+)(?=`+)|(?<!\\)`)"#, "`code`", true);
    check(r#"\\(.)"#, "\\*", true);
    check(r#"<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>"#, "<http://x.com>", true);
    check(r#"<([^<> !]+@[^@<> ]+)>"#, "<a@b.com>", true);
    check(r#"  \n"#, "x  \ny", true);
    check(r#"(&(?:\#[0-9]+|\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)"#, "&#38;", true);
    check(
        r#"\(\s*(?:(<[^<>]*>)\s*(?:('[^']*'|"[^"]*")\s*)?\))?"#,
        "(http://x.com \"t\")",
        true,
    );
    // delimiter boundary (smart `*`)
    check(
        r#"(?x)(?P<ambiguous>(?<!^)(?<![\s\*])(?:(?<=_)|(?<![\w\*]))\*+(?![\s\*])(?!$))|(?P<end>(?<!^)(?<![\s\*])\*+)|(?P<start>(?:(?<=_)|(?<![\w\*]))\*+(?![\s\*])(?!$))"#,
        "*em*",
        true,
    );
    // html inline (possessive quantifiers)
    check(
        r#"(<(\/?+[a-zA-Z][^\s\"'<>@]*+(?:\s+[^\s\"'=<>]++(?:\s*+=\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\s\"'=<>]++))?+)*+\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|[!]\[CDATA\[(?:(?!<![CDATA[|\]\]>).)*\]\]*)>)"#,
        "<a href=\"x\">",
        true,
    );
    // unescape / placeholders

    check("\u{2}(\\d+)\u{3}", "\u{2}92\u{3}", true);
    check("\u{2}klzzwxh:([0-9]+)\u{3}", "\u{2}klzzwxh:0004\u{3}", true);
    // postprocessors / serializers
    check(r#"^\<\/?([^ >]+)"#, "<div>", true);
    check(r#"&(?!(?:\#[0-9]+|\#x[0-9a-f]+|[0-9a-z]+);)"#, "a & b", true);
    // extensions (spot)
    check(r#"^[ ]{0,3}\|"#, "| a | b |", true);
}
