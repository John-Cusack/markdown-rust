//! Machine-generated upstream regex sources (`scripts/dump_patterns.py`).
//! DO NOT EDIT: fidelity of every pattern is covered by
//! `markdown-core/tests/patterns_diff.rs` against `tests/data/`.

/// upstream `blk_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_BLK_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `blk_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_BLK_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `blk_HashHeaderProcessor_RE` (flags `-`)
pub const RE_BLK_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `blk_ReferenceProcessor_RE` (flags `m`)
pub const RE_BLK_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `blk_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_BLK_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `codehilite_shebang` (flags `x`)
pub const RE_CODEHILITE_SHEBANG: &str = "(?x)\n(?:(?:^::+)|(?P<shebang>^[#]!))\n(?P<path>(?:/\\w+)*[/ ])?\n(?P<lang>[\\w#.+-]*)\n\\s*\n(hl_lines=(?P<quot>\"|')(?P<hl_lines>.*?)(?P=quot))?\n";
/// upstream `delim_star_dumb_bad` (flags `-`)
pub const RE_DELIM_STAR_DUMB_BAD: &str = "\\*+";
/// upstream `delim_star_dumb_boundary` (flags `x`)
pub const RE_DELIM_STAR_DUMB_BOUNDARY: &str = "(?x)(?x)\n                (?P<ambiguous>(?<!^)(?<![\\s\\*])\\*{1,}(?![\\s\\*])(?!$))|\n                (?P<end>(?<!^)(?<![\\s\\*])\\*{1,})|\n                (?P<start>\\*{1,}(?![\\s\\*])(?!$))\n                ";
/// upstream `delim_star_smart_bad` (flags `-`)
pub const RE_DELIM_STAR_SMART_BAD: &str = "\\*+";
/// upstream `delim_star_smart_boundary` (flags `x`)
pub const RE_DELIM_STAR_SMART_BOUNDARY: &str = "(?x)(?x)\n                (?P<ambiguous>(?<!^)(?<![\\s\\*])(?:(?<=_)|(?<![\\w\\*]))\\*{1,}(?:(?=_)|(?![\\w\\*]))(?![\\s\\*])(?!$))|\n                (?P<end>(?<!^)(?<![\\s\\*])\\*{1,}(?:(?=_)|(?![\\w\\*])))|\n                (?P<start>(?:(?<=_)|(?<![\\w\\*]))\\*{1,}(?![\\s\\*])(?!$))\n                ";
/// upstream `delim_us_dumb_bad` (flags `-`)
pub const RE_DELIM_US_DUMB_BAD: &str = "_+";
/// upstream `delim_us_dumb_boundary` (flags `x`)
pub const RE_DELIM_US_DUMB_BOUNDARY: &str = "(?x)(?x)\n                (?P<ambiguous>(?<!^)(?<![\\s_])_{1,}(?![\\s_])(?!$))|\n                (?P<end>(?<!^)(?<![\\s_])_{1,})|\n                (?P<start>_{1,}(?![\\s_])(?!$))\n                ";
/// upstream `delim_us_smart_bad` (flags `-`)
pub const RE_DELIM_US_SMART_BAD: &str = "_+";
/// upstream `delim_us_smart_boundary` (flags `x`)
pub const RE_DELIM_US_SMART_BOUNDARY: &str = "(?x)(?x)\n                (?P<ambiguous>(?<!^)(?<![\\s_])(?<![\\w_])_{1,}(?![\\w_])(?![\\s_])(?!$))|\n                (?P<end>(?<!^)(?<![\\s_])_{1,}(?![\\w_]))|\n                (?P<start>(?<![\\w_])_{1,}(?![\\s_])(?!$))\n                ";
/// upstream `ext_abbr_AbbrBlockprocessor_RE` (flags `m`)
pub const RE_EXT_ABBR_ABBRBLOCKPROCESSOR_RE: &str = "(?m)^[*]\\[(?P<abbr>[^\\\\]*?)\\][ ]?:[ ]*\\n?[ ]*(?P<title>.*)$";
/// upstream `ext_admonition_AdmonitionProcessor_RE` (flags `-`)
pub const RE_EXT_ADMONITION_ADMONITIONPROCESSOR_RE: &str = "(?:^|\\n)!!! ?([\\w\\-]+(?: +[\\w\\-]+)*)(?: +\"(.*?)\")? *(?:\\n|$)";
/// upstream `ext_admonition_AdmonitionProcessor_RE_SPACES` (flags `-`)
pub const RE_EXT_ADMONITION_ADMONITIONPROCESSOR_RE_SPACES: &str = "  +";
/// upstream `ext_attr_list_AttrListTreeprocessor_BLOCK_RE` (flags `-`)
pub const RE_EXT_ATTR_LIST_ATTRLISTTREEPROCESSOR_BLOCK_RE: &str = "\\n[ ]*\\{\\:?[ ]*([^\\}\\n ][^\\n]*)[ ]*\\}[ ]*$";
/// upstream `ext_attr_list_AttrListTreeprocessor_HEADER_RE` (flags `-`)
pub const RE_EXT_ATTR_LIST_ATTRLISTTREEPROCESSOR_HEADER_RE: &str = "[ ]+\\{\\:?[ ]*([^\\}\\n ][^\\n]*)[ ]*\\}[ ]*$";
/// upstream `ext_attr_list_AttrListTreeprocessor_INLINE_RE` (flags `-`)
pub const RE_EXT_ATTR_LIST_ATTRLISTTREEPROCESSOR_INLINE_RE: &str = "^\\{\\:?[ ]*([^\\}\\n ][^\\n]*)[ ]*\\}";
/// upstream `ext_attr_list_AttrListTreeprocessor_NAME_RE` (flags `-`)
pub const RE_EXT_ATTR_LIST_ATTRLISTTREEPROCESSOR_NAME_RE: &str = "[^A-Z_a-z\\u00c0-\\u00d6\\u00d8-\\u00f6\\u00f8-\\u02ff\\u0370-\\u037d\\u037f-\\u1fff\\u200c-\\u200d\\u2070-\\u218f\\u2c00-\\u2fef\\u3001-\\ud7ff\\uf900-\\ufdcf\\ufdf0-\\ufffd\\:\\-\\.0-9\\u00b7\\u0300-\\u036f\\u203f-\\u2040]+";
/// upstream `ext_attr_list_NAME_RE` (flags `-`)
pub const RE_EXT_ATTR_LIST_NAME_RE: &str = "[^A-Z_a-z\\u00c0-\\u00d6\\u00d8-\\u00f6\\u00f8-\\u02ff\\u0370-\\u037d\\u037f-\\u1fff\\u200c-\\u200d\\u2070-\\u218f\\u2c00-\\u2fef\\u3001-\\ud7ff\\uf900-\\ufdcf\\ufdf0-\\ufffd\\:\\-\\.0-9\\u00b7\\u0300-\\u036f\\u203f-\\u2040]+";
/// upstream `ext_def_list_DefListProcessor_NO_INDENT_RE` (flags `-`)
pub const RE_EXT_DEF_LIST_DEFLISTPROCESSOR_NO_INDENT_RE: &str = "^[ ]{0,3}[^ :]";
/// upstream `ext_def_list_DefListProcessor_RE` (flags `-`)
pub const RE_EXT_DEF_LIST_DEFLISTPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}:[ ]{1,3}(.*?)(\\n|$)";
/// upstream `ext_fenced_code_FencedBlockPreprocessor_FENCED_BLOCK_RE` (flags `msx`)
pub const RE_EXT_FENCED_CODE_FENCEDBLOCKPREPROCESSOR_FENCED_BLOCK_RE: &str = "(?msx)\n(?P<fence>^(?:~{3,}|`{3,}))[ ]*                          # opening fence\n((\\{(?P<attrs>[^\\n]*)\\})|                                # (optional {attrs} or\n(\\.?(?P<lang>[\\w#.+-]*)[ ]*)?                            # optional (.)lang\n(hl_lines=(?P<quot>\"|')(?P<hl_lines>.*?)(?P=quot)[ ]*)?) # optional hl_lines)\n\\n                                                       # newline (end of opening fence)\n(?P<code>.*?)(?<=\\n)                                     # the code block\n(?P=fence)[ ]*$                                          # closing fence\n";
/// upstream `ext_footnotes_FootnoteBlockProcessor_RE` (flags `m`)
pub const RE_EXT_FOOTNOTES_FOOTNOTEBLOCKPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[\\^([^\\]]*)\\]:[ ]*(.*)$";
/// upstream `ext_footnotes_RE_REFERENCE` (flags `-`)
pub const RE_EXT_FOOTNOTES_RE_REFERENCE: &str = "(?<!!)\\[\\^([^\\]]*)\\](?!\\s*:)";
/// upstream `ext_footnotes_RE_REF_ID` (flags `-`)
pub const RE_EXT_FOOTNOTES_RE_REF_ID: &str = "(fnref)(\\d+)";
/// upstream `ext_legacy_attrs_ATTR_RE` (flags `-`)
pub const RE_EXT_LEGACY_ATTRS_ATTR_RE: &str = "\\{@([^\\}]*)=([^\\}]*)}";
/// upstream `ext_legacy_em_DelimiterProcessor_SPACE` (flags `-`)
pub const RE_EXT_LEGACY_EM_DELIMITERPROCESSOR_SPACE: &str = "\\s";
/// upstream `ext_md_in_html_RawHtmlPostprocessor_BLOCK_LEVEL_REGEX` (flags `-`)
pub const RE_EXT_MD_IN_HTML_RAWHTMLPOSTPROCESSOR_BLOCK_LEVEL_REGEX: &str = "^<\\/?([^ >]+)";
/// upstream `ext_md_in_html_blank_line_re` (flags `-`)
pub const RE_EXT_MD_IN_HTML_BLANK_LINE_RE: &str = "^([ ]*\\n){2}";
/// upstream `ext_meta_BEGIN_RE` (flags `-`)
pub const RE_EXT_META_BEGIN_RE: &str = "^-{3}(\\s.*)?";
/// upstream `ext_meta_END_RE` (flags `-`)
pub const RE_EXT_META_END_RE: &str = "^(-{3}|\\.{3})(\\s.*)?";
/// upstream `ext_meta_META_MORE_RE` (flags `-`)
pub const RE_EXT_META_META_MORE_RE: &str = "^[ ]{4,}(?P<value>.*)";
/// upstream `ext_meta_META_RE` (flags `-`)
pub const RE_EXT_META_META_RE: &str = "^[ ]{0,3}(?P<key>[A-Za-z0-9_-]+):\\s*(?P<value>.*)";
/// upstream `ext_tables_TableProcessor_RE_CODE_PIPES` (flags `-`)
pub const RE_EXT_TABLES_TABLEPROCESSOR_RE_CODE_PIPES: &str = "(?:(\\\\\\\\)|(\\\\`+)|(`+)|(\\\\\\|)|(\\|))";
/// upstream `ext_tables_TableProcessor_RE_END_BORDER` (flags `-`)
pub const RE_EXT_TABLES_TABLEPROCESSOR_RE_END_BORDER: &str = "(?<!\\\\)(?:\\\\\\\\)*\\|$";
/// upstream `ext_toc_HTML_PLACEHOLDER_RE` (flags `-`)
pub const RE_EXT_TOC_HTML_PLACEHOLDER_RE: &str = "\u{2}wzxhzdk:([0-9]+)\u{3}";
/// upstream `ext_toc_IDCOUNT_RE` (flags `-`)
pub const RE_EXT_TOC_IDCOUNT_RE: &str = "^(.*)_([0-9]+)$";
/// upstream `ext_toc_RE_AMP` (flags `i`)
pub const RE_EXT_TOC_RE_AMP: &str = "(?i)&(?!(?:\\#[0-9]+|\\#x[0-9a-f]+|[0-9a-z]+);)";
/// upstream `ext_toc_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXT_TOC_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_abbr_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_abbr_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_abbr_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ABBR_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_abbr_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_abbr_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_ABBR_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_abbr_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_abbr_03_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_ABBR_03_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_abbr_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_abbr_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_ABBR_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_abbr_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_ABBR_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_abbr_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_abbr_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_ABBR_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_abbr_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ABBR_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_abbr_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_ABBR_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_abbr_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_abbr_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_abbr_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_ABBR_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_abbr_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ABBR_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_abbr_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_ABBR_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_abbr_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_abbr_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_ABBR_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_abbr_09_AbbrBlockprocessor_RE` (flags `m`)
pub const RE_EXTINST_ABBR_09_ABBRBLOCKPROCESSOR_RE: &str = "(?m)^[*]\\[(?P<abbr>[^\\\\]*?)\\][ ]?:[ ]*\\n?[ ]*(?P<title>.*)$";
/// upstream `extinst_abbr_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_abbr_10_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_ABBR_10_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_abbr_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_abbr_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_abbr_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_abbr_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ABBR_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_admonition_00_AdmonitionProcessor_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_00_ADMONITIONPROCESSOR_RE: &str = "(?:^|\\n)!!! ?([\\w\\-]+(?: +[\\w\\-]+)*)(?: +\"(.*?)\")? *(?:\\n|$)";
/// upstream `extinst_admonition_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_admonition_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_admonition_02_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_02_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_admonition_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_admonition_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_admonition_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_admonition_04_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_04_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_admonition_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_admonition_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_admonition_05_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_ADMONITION_05_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_admonition_06_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_ADMONITION_06_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_admonition_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_admonition_07_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_07_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_admonition_07_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_07_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_admonition_07_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_07_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_admonition_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_admonition_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_admonition_08_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_08_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_admonition_08_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_08_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_admonition_08_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_08_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_admonition_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_admonition_09_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_ADMONITION_09_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_admonition_10_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_ADMONITION_10_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_admonition_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_admonition_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_admonition_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_admonition_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ADMONITION_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_attr_list_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_attr_list_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_attr_list_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_attr_list_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_attr_list_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_attr_list_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_attr_list_03_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_03_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_attr_list_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_attr_list_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_ATTR_LIST_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_attr_list_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_ATTR_LIST_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_attr_list_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_attr_list_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_attr_list_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_attr_list_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_attr_list_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_attr_list_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_attr_list_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_attr_list_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_attr_list_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_attr_list_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_attr_list_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_ATTR_LIST_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_attr_list_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_attr_list_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_ATTR_LIST_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_attr_list_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_attr_list_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_attr_list_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_attr_list_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_ATTR_LIST_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_codehilite_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_codehilite_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_codehilite_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_codehilite_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_codehilite_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_codehilite_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_codehilite_03_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_03_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_codehilite_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_codehilite_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_CODEHILITE_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_codehilite_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_CODEHILITE_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_codehilite_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_codehilite_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_codehilite_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_codehilite_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_codehilite_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_codehilite_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_codehilite_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_codehilite_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_codehilite_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_codehilite_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_codehilite_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_CODEHILITE_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_codehilite_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_codehilite_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_CODEHILITE_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_codehilite_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_codehilite_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_codehilite_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_codehilite_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_CODEHILITE_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_def_list_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_def_list_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_def_list_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_def_list_02_DefListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_02_DEFLISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_def_list_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_def_list_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_def_list_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_def_list_04_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_04_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_def_list_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_def_list_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_def_list_05_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_DEF_LIST_05_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_def_list_06_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_DEF_LIST_06_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_def_list_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_def_list_07_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_07_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_def_list_07_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_07_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_def_list_07_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_07_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_def_list_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_def_list_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_def_list_08_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_08_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_def_list_08_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_08_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_def_list_08_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_08_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_def_list_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_def_list_09_DefListProcessor_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_09_DEFLISTPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}:[ ]{1,3}(.*?)(\\n|$)";
/// upstream `extinst_def_list_10_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_DEF_LIST_10_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_def_list_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_def_list_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_def_list_11_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_DEF_LIST_11_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_def_list_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_def_list_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_DEF_LIST_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_fenced_code_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_fenced_code_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_fenced_code_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_fenced_code_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_fenced_code_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_fenced_code_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_fenced_code_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_fenced_code_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_fenced_code_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_FENCED_CODE_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_fenced_code_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_FENCED_CODE_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_fenced_code_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_fenced_code_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_fenced_code_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_fenced_code_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_fenced_code_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_fenced_code_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_fenced_code_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_fenced_code_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_fenced_code_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_fenced_code_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_fenced_code_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_FENCED_CODE_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_fenced_code_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_fenced_code_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_FENCED_CODE_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_fenced_code_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_fenced_code_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_fenced_code_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_fenced_code_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FENCED_CODE_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_footnotes_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_footnotes_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_footnotes_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_footnotes_02_FootnoteInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_02_FOOTNOTEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\[\\^([^\\]]*)\\]";
/// upstream `extinst_footnotes_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_footnotes_03_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_03_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_footnotes_04_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_04_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_footnotes_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_FOOTNOTES_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_footnotes_04_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_04_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_footnotes_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_FOOTNOTES_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_footnotes_05_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_05_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_footnotes_06_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_06_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_footnotes_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_footnotes_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_footnotes_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_footnotes_07_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_07_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_footnotes_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_footnotes_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_footnotes_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_footnotes_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_FOOTNOTES_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_footnotes_08_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_08_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_footnotes_09_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_09_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_footnotes_09_FootnoteBlockProcessor_RE` (flags `m`)
pub const RE_EXTINST_FOOTNOTES_09_FOOTNOTEBLOCKPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[\\^([^\\]]*)\\]:[ ]*(.*)$";
/// upstream `extinst_footnotes_10_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_10_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_footnotes_10_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_FOOTNOTES_10_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_footnotes_11_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_11_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_footnotes_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_footnotes_13_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_13_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_footnotes_14_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_FOOTNOTES_14_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_legacy_attrs_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_legacy_attrs_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_legacy_attrs_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_legacy_attrs_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_legacy_attrs_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_legacy_attrs_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_legacy_attrs_03_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_03_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_legacy_attrs_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_legacy_attrs_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_LEGACY_ATTRS_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_legacy_attrs_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_LEGACY_ATTRS_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_legacy_attrs_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_legacy_attrs_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_legacy_attrs_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_legacy_attrs_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_legacy_attrs_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_legacy_attrs_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_legacy_attrs_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_legacy_attrs_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_legacy_attrs_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_legacy_attrs_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_legacy_attrs_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_ATTRS_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_legacy_attrs_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_legacy_attrs_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_LEGACY_ATTRS_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_legacy_attrs_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_legacy_attrs_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_legacy_attrs_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_legacy_attrs_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_ATTRS_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_legacy_em_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_legacy_em_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_legacy_em_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_legacy_em_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_legacy_em_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_legacy_em_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_legacy_em_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_legacy_em_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_legacy_em_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_LEGACY_EM_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_legacy_em_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_LEGACY_EM_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_legacy_em_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_legacy_em_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_legacy_em_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_legacy_em_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_legacy_em_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_legacy_em_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_legacy_em_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_legacy_em_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_legacy_em_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_legacy_em_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_legacy_em_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_LEGACY_EM_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_legacy_em_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_legacy_em_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_LEGACY_EM_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_legacy_em_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_legacy_em_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_legacy_em_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_legacy_em_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_LEGACY_EM_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_md_in_html_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_md_in_html_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_md_in_html_02_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_02_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_md_in_html_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_md_in_html_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_md_in_html_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_md_in_html_04_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_04_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_md_in_html_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_md_in_html_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_md_in_html_05_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_MD_IN_HTML_05_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_md_in_html_06_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_MD_IN_HTML_06_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_md_in_html_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_md_in_html_07_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_07_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_md_in_html_07_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_07_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_md_in_html_07_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_07_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_md_in_html_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_md_in_html_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_md_in_html_08_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_08_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_md_in_html_08_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_08_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_md_in_html_08_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_08_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_md_in_html_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_md_in_html_09_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_MD_IN_HTML_09_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_md_in_html_10_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_MD_IN_HTML_10_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_md_in_html_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_md_in_html_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_md_in_html_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_md_in_html_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_MD_IN_HTML_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_meta_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_meta_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_meta_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_META_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_meta_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_meta_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_META_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_meta_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_META_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_meta_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_meta_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_meta_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_META_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_meta_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_META_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_meta_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_meta_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_META_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_meta_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_META_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_meta_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_META_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_meta_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_meta_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_meta_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_META_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_meta_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_META_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_meta_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_META_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_meta_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_meta_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_META_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_meta_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_meta_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_META_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_meta_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_meta_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_meta_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_meta_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_META_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_nl2br_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_nl2br_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_nl2br_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_nl2br_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_nl2br_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_nl2br_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_nl2br_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_nl2br_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_nl2br_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_NL2BR_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_nl2br_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_NL2BR_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_nl2br_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_nl2br_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_nl2br_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_nl2br_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_nl2br_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_nl2br_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_nl2br_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_nl2br_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_nl2br_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_nl2br_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_nl2br_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_NL2BR_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_nl2br_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_nl2br_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_NL2BR_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_nl2br_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_nl2br_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_nl2br_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_nl2br_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_nl2br_14_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_NL2BR_14_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\n";
/// upstream `extinst_sane_lists_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_sane_lists_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_sane_lists_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_sane_lists_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_sane_lists_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_sane_lists_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_sane_lists_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_sane_lists_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_sane_lists_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_SANE_LISTS_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_sane_lists_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_SANE_LISTS_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_sane_lists_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_sane_lists_06_SaneOListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_06_SANEOLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.))[ ]+(.*)";
/// upstream `extinst_sane_lists_06_SaneOListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_06_SANEOLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_sane_lists_06_SaneOListProcessor_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_06_SANEOLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_sane_lists_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_sane_lists_07_SaneUListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_07_SANEULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}(([*+-]))[ ]+(.*)";
/// upstream `extinst_sane_lists_07_SaneUListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_07_SANEULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_sane_lists_07_SaneUListProcessor_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_07_SANEULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_sane_lists_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_sane_lists_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_sane_lists_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_SANE_LISTS_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_sane_lists_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_sane_lists_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_SANE_LISTS_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_sane_lists_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_sane_lists_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_sane_lists_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_sane_lists_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SANE_LISTS_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_smarty_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_smarty_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_smarty_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_smarty_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_smarty_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_smarty_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_smarty_03_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_03_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_smarty_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_smarty_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_SMARTY_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_smarty_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_SMARTY_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_smarty_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_smarty_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_smarty_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_smarty_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_smarty_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_smarty_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_smarty_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_smarty_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_smarty_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_smarty_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_smarty_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_SMARTY_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_smarty_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_smarty_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_SMARTY_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_smarty_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_smarty_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_smarty_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_smarty_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_SMARTY_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_tables_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_tables_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_tables_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_TABLES_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_tables_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_tables_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_TABLES_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_tables_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_tables_04_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_TABLES_04_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_tables_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_tables_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_tables_05_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_TABLES_05_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_tables_06_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_TABLES_06_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_tables_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_tables_07_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_TABLES_07_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_tables_07_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_TABLES_07_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_tables_07_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_TABLES_07_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_tables_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_tables_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_tables_08_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_TABLES_08_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_tables_08_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_TABLES_08_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_tables_08_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_TABLES_08_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_tables_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_tables_09_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_TABLES_09_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_tables_10_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_TABLES_10_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_tables_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_tables_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_tables_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_tables_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TABLES_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_toc_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_toc_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_toc_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_TOC_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_toc_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_toc_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_TOC_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_toc_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_toc_03_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_TOC_03_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_toc_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_toc_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_TOC_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_toc_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_TOC_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_toc_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_toc_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_TOC_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_toc_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_TOC_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_toc_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_TOC_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_toc_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_toc_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_toc_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_TOC_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_toc_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_TOC_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_toc_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_TOC_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_toc_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_toc_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_TOC_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_toc_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_toc_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_TOC_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_toc_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_toc_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_toc_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_toc_13_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_TOC_13_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `extinst_wikilinks_00_BacktickInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_00_BACKTICKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `extinst_wikilinks_01_EscapeInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_01_ESCAPEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\\\(.)";
/// upstream `extinst_wikilinks_01_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_01_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `extinst_wikilinks_02_ReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_02_REFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_wikilinks_02_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_02_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `extinst_wikilinks_03_HashHeaderProcessor_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_03_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `extinst_wikilinks_03_LinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_03_LINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_wikilinks_04_ImageInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_04_IMAGEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_wikilinks_04_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_EXTINST_WIKILINKS_04_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `extinst_wikilinks_05_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_EXTINST_WIKILINKS_05_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `extinst_wikilinks_05_ImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_05_IMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_wikilinks_06_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_06_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_wikilinks_06_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_06_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_wikilinks_06_OListProcessor_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_06_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `extinst_wikilinks_06_ShortReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_06_SHORTREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(?<!\\!)\\[";
/// upstream `extinst_wikilinks_07_ShortImageReferenceInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_07_SHORTIMAGEREFERENCEINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\!\\[";
/// upstream `extinst_wikilinks_07_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_07_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `extinst_wikilinks_07_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_07_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `extinst_wikilinks_07_UListProcessor_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_07_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `extinst_wikilinks_08_AutolinkInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_08_AUTOLINKINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `extinst_wikilinks_08_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_EXTINST_WIKILINKS_08_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `extinst_wikilinks_09_AutomailInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_09_AUTOMAILINLINEPROCESSOR_COMPILED_RE: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `extinst_wikilinks_09_ReferenceProcessor_RE` (flags `m`)
pub const RE_EXTINST_WIKILINKS_09_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `extinst_wikilinks_10_SubstituteTagInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_10_SUBSTITUTETAGINLINEPROCESSOR_COMPILED_RE: &str = "(?s)  \\n";
/// upstream `extinst_wikilinks_11_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_11_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `extinst_wikilinks_12_HtmlInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_12_HTMLINLINEPROCESSOR_COMPILED_RE: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `extinst_wikilinks_13_WikiLinksInlineProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_13_WIKILINKSINLINEPROCESSOR_COMPILED_RE: &str = "(?s)\\[\\[([\\w0-9_ -]+)\\]\\]";
/// upstream `extinst_wikilinks_14_DelimiterProcessor_compiled_re` (flags `s`)
pub const RE_EXTINST_WIKILINKS_14_DELIMITERPROCESSOR_COMPILED_RE: &str = "(?s)\\*|_";
/// upstream `html_blank_line_re` (flags `-`)
pub const RE_HTML_BLANK_LINE_RE: &str = "^([ ]*\\n){2}";
/// upstream `html_commentclose` (flags `-`)
pub const RE_HTML_COMMENTCLOSE: &str = "--!?>";
/// upstream `html_inner_blank_line_re` (flags `-`)
pub const RE_HTML_INNER_BLANK_LINE_RE: &str = "\\n[ \\t]*\\n";
/// upstream `html_std_attrfind_tolerant` (flags `x`)
pub const RE_HTML_STD_ATTRFIND_TOLERANT: &str = "(?x)\n  (\n    (?<=['\"\\t\\n\\r\\f /])[^\\t\\n\\r\\f />][^\\t\\n\\r\\f /=>]*  # attribute name\n   )\n  ([\\t\\n\\r\\f ]*=[\\t\\n\\r\\f ]*        # value indicator\n    ('[^']*'                        # LITA-enclosed value\n    |\"[^\"]*\"                        # LIT-enclosed value\n    |(?!['\"])[^>\\t\\n\\r\\f ]*         # bare value\n    )\n   )?\n  (?:[\\t\\n\\r\\f ]|/(?!>))*           # possibly followed by a space\n";
/// upstream `html_std_locatestarttagend_tolerant` (flags `x`)
pub const RE_HTML_STD_LOCATESTARTTAGEND_TOLERANT: &str = "(?x)\n  <[a-zA-Z][^`\\t\\n\\r\\f />\\x00]*       # tag name <= added backtick here\n  (?:[\\s/]*                           # optional whitespace before attribute name\n    (?:(?<=['\"\\s/])[^`\\s/>][^\\s/=>]*  # attribute name <= added backtick here\n      (?:\\s*=+\\s*                     # value indicator\n        (?:'[^']*'                    # LITA-enclosed value\n          |\"[^\"]*\"                    # LIT-enclosed value\n          |(?!['\"])[^`>\\s]*           # bare value <= added backtick here\n         )\n         (?:\\s*,)*                    # possibly followed by a comma\n       )?(?:\\s|/(?!>))*\n     )*\n   )?\n  \\s*                                 # trailing whitespace\n";
/// upstream `html_std_locatetagend` (flags `x`)
pub const RE_HTML_STD_LOCATETAGEND: &str = "(?x)\n  [a-zA-Z][^`\\t\\n\\r\\f />]*           # tag name\n  [\\t\\n\\r\\f /]*                     # optional whitespace before attribute name\n  (?:(?<=['\"\\t\\n\\r\\f /])[^`\\t\\n\\r\\f />][^\\t\\n\\r\\f /=>]*  # attribute name\n    (?:=                            # value indicator\n      (?:'[^']*'                    # LITA-enclosed value\n        |\"[^\"]*\"                    # LIT-enclosed value\n        |(?!['\"])[^>\\t\\n\\r\\f ]*     # bare value\n       )\n     )?\n    [\\t\\n\\r\\f /]*                   # possibly followed by a space\n   )*\n   >?\n";
/// upstream `html_std_tagfind_tolerant` (flags `-`)
pub const RE_HTML_STD_TAGFIND_TOLERANT: &str = "([a-zA-Z][^\\t\\n\\r\\f />]*)(?:[\\t\\n\\r\\f ]|/(?!>))*";
/// upstream `il_DelimiterProcessor_SPACE` (flags `-`)
pub const RE_IL_DELIMITERPROCESSOR_SPACE: &str = "\\s";
/// upstream `il_LinkInlineProcessor_RE_LINK` (flags `s`)
pub const RE_IL_LINKINLINEPROCESSOR_RE_LINK: &str = "(?s)\\(\\s*(?:(<[^<>]*>)\\s*(?:('[^']*'|\"[^\"]*\")\\s*)?\\))?";
/// upstream `il_LinkInlineProcessor_RE_TITLE_CLEAN` (flags `-`)
pub const RE_IL_LINKINLINEPROCESSOR_RE_TITLE_CLEAN: &str = "\\s";
/// upstream `il_ReferenceInlineProcessor_NEWLINE_CLEANUP_RE` (flags `m`)
pub const RE_IL_REFERENCEINLINEPROCESSOR_NEWLINE_CLEANUP_RE: &str = "(?m)\\s+";
/// upstream `il_ReferenceInlineProcessor_RE_LINK` (flags `s`)
pub const RE_IL_REFERENCEINLINEPROCESSOR_RE_LINK: &str = "(?s)\\s?\\[([^\\]]*)\\]";
/// upstream `inst_blk_BlockQuoteProcessor_RE` (flags `-`)
pub const RE_INST_BLK_BLOCKQUOTEPROCESSOR_RE: &str = "(^|\\n)[ ]{0,3}>[ ]?(.*)";
/// upstream `inst_blk_HRProcessor_SEARCH_RE` (flags `m`)
pub const RE_INST_BLK_HRPROCESSOR_SEARCH_RE: &str = "(?m)^[ ]{0,3}(?=(?P<atomicgroup>(-+[ ]{0,2}){3,}|(_+[ ]{0,2}){3,}|(\\*+[ ]{0,2}){3,}))(?P=atomicgroup)[ ]*$";
/// upstream `inst_blk_HashHeaderProcessor_RE` (flags `-`)
pub const RE_INST_BLK_HASHHEADERPROCESSOR_RE: &str = "(?:^|\\n)(?P<level>#{1,6})(?P<header>(?:\\\\.|[^\\\\])*?)#*(?:\\n|$)";
/// upstream `inst_blk_ListIndentProcessor_INDENT_RE` (flags `-`)
pub const RE_INST_BLK_LISTINDENTPROCESSOR_INDENT_RE: &str = "^(([ ]{4})+)";
/// upstream `inst_blk_OListProcessor_CHILD_RE` (flags `-`)
pub const RE_INST_BLK_OLISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `inst_blk_OListProcessor_INDENT_RE` (flags `-`)
pub const RE_INST_BLK_OLISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `inst_blk_OListProcessor_RE` (flags `-`)
pub const RE_INST_BLK_OLISTPROCESSOR_RE: &str = "^[ ]{0,3}\\d+\\.[ ]+(.*)";
/// upstream `inst_blk_ReferenceProcessor_RE` (flags `m`)
pub const RE_INST_BLK_REFERENCEPROCESSOR_RE: &str = "(?m)^[ ]{0,3}\\[([^\\[\\]]*)\\]:[ ]*(?:\\n[ ]*)?([^\\s]+)[ ]*(?:\\n[ ]*)?(([\"\\'])(.*)\\4[ ]*|\\((.*)\\)[ ]*)?$";
/// upstream `inst_blk_SetextHeaderProcessor_RE` (flags `m`)
pub const RE_INST_BLK_SETEXTHEADERPROCESSOR_RE: &str = "(?m)^.*?\\n(?:=+|-+)[ ]*(\\n|$)";
/// upstream `inst_blk_UListProcessor_CHILD_RE` (flags `-`)
pub const RE_INST_BLK_ULISTPROCESSOR_CHILD_RE: &str = "^[ ]{0,3}((\\d+\\.)|[*+-])[ ]+(.*)";
/// upstream `inst_blk_UListProcessor_INDENT_RE` (flags `-`)
pub const RE_INST_BLK_ULISTPROCESSOR_INDENT_RE: &str = "^[ ]{4,7}((\\d+\\.)|[*+-])[ ]+.*";
/// upstream `inst_blk_UListProcessor_RE` (flags `-`)
pub const RE_INST_BLK_ULISTPROCESSOR_RE: &str = "^[ ]{0,3}[*+-][ ]+(.*)";
/// upstream `inst_delim_bad_star` (flags `-`)
pub const RE_INST_DELIM_BAD_STAR: &str = "\\*+";
/// upstream `inst_delim_bad_us` (flags `-`)
pub const RE_INST_DELIM_BAD_US: &str = "_+";
/// upstream `inst_delim_boundary_star` (flags `x`)
pub const RE_INST_DELIM_BOUNDARY_STAR: &str = "(?x)(?x)\n                (?P<ambiguous>(?<!^)(?<![\\s\\*])\\*{1,}(?![\\s\\*])(?!$))|\n                (?P<end>(?<!^)(?<![\\s\\*])\\*{1,})|\n                (?P<start>\\*{1,}(?![\\s\\*])(?!$))\n                ";
/// upstream `inst_delim_boundary_us` (flags `x`)
pub const RE_INST_DELIM_BOUNDARY_US: &str = "(?x)(?x)\n                (?P<ambiguous>(?<!^)(?<![\\s_])(?<![\\w_])_{1,}(?![\\w_])(?![\\s_])(?!$))|\n                (?P<end>(?<!^)(?<![\\s_])_{1,}(?![\\w_]))|\n                (?P<start>(?<![\\w_])_{1,}(?![\\s_])(?!$))\n                ";
/// upstream `inst_il_00_BacktickInlineProcessor` (flags `s`)
pub const RE_INST_IL_00_BACKTICKINLINEPROCESSOR: &str = "(?s)(?:(?<!\\\\)((?:\\\\{2})+)(?=`+)|(?<!\\\\)`)";
/// upstream `inst_il_01_EscapeInlineProcessor` (flags `s`)
pub const RE_INST_IL_01_ESCAPEINLINEPROCESSOR: &str = "(?s)\\\\(.)";
/// upstream `inst_il_02_ReferenceInlineProcessor` (flags `s`)
pub const RE_INST_IL_02_REFERENCEINLINEPROCESSOR: &str = "(?s)(?<!\\!)\\[";
/// upstream `inst_il_03_LinkInlineProcessor` (flags `s`)
pub const RE_INST_IL_03_LINKINLINEPROCESSOR: &str = "(?s)(?<!\\!)\\[";
/// upstream `inst_il_04_ImageInlineProcessor` (flags `s`)
pub const RE_INST_IL_04_IMAGEINLINEPROCESSOR: &str = "(?s)\\!\\[";
/// upstream `inst_il_05_ImageReferenceInlineProcessor` (flags `s`)
pub const RE_INST_IL_05_IMAGEREFERENCEINLINEPROCESSOR: &str = "(?s)\\!\\[";
/// upstream `inst_il_06_ShortReferenceInlineProcessor` (flags `s`)
pub const RE_INST_IL_06_SHORTREFERENCEINLINEPROCESSOR: &str = "(?s)(?<!\\!)\\[";
/// upstream `inst_il_07_ShortImageReferenceInlineProcessor` (flags `s`)
pub const RE_INST_IL_07_SHORTIMAGEREFERENCEINLINEPROCESSOR: &str = "(?s)\\!\\[";
/// upstream `inst_il_08_AutolinkInlineProcessor` (flags `s`)
pub const RE_INST_IL_08_AUTOLINKINLINEPROCESSOR: &str = "(?s)<((?:[Ff]|[Hh][Tt])[Tt][Pp][Ss]?://[^<>]*)>";
/// upstream `inst_il_09_AutomailInlineProcessor` (flags `s`)
pub const RE_INST_IL_09_AUTOMAILINLINEPROCESSOR: &str = "(?s)<([^<> !]+@[^@<> ]+)>";
/// upstream `inst_il_10_SubstituteTagInlineProcessor` (flags `s`)
pub const RE_INST_IL_10_SUBSTITUTETAGINLINEPROCESSOR: &str = "(?s)  \\n";
/// upstream `inst_il_11_HtmlInlineProcessor` (flags `s`)
pub const RE_INST_IL_11_HTMLINLINEPROCESSOR: &str = "(?s)(<(\\/?+[a-zA-Z][^\\s\"'<>@]*+(?:\\s+[^\\s\"'=<>]++(?:\\s*+=\\s*+(?:\"[^\"]*+\"|'[^']*+'|[^\\s\"'=<>]++))?+)*+\\s*+/?|!--(?:(?!<!--|-->).)*--|[?](?:(?!<[?]|[?]>).)*[?]|!\\[CDATA\\[(?:(?!<!\\[CDATA\\[|\\]\\]>).)*\\]\\])>)";
/// upstream `inst_il_12_HtmlInlineProcessor` (flags `s`)
pub const RE_INST_IL_12_HTMLINLINEPROCESSOR: &str = "(?s)(&(?:\\#[0-9]+|\\#x[0-9a-fA-F]+|[a-zA-Z0-9]+);)";
/// upstream `inst_il_13_DelimiterProcessor` (flags `s`)
pub const RE_INST_IL_13_DELIMITERPROCESSOR: &str = "(?s)\\*|_";
/// upstream `inst_pp_RawHtmlPostprocessor_BLOCK_LEVEL_REGEX` (flags `-`)
pub const RE_INST_PP_RAWHTMLPOSTPROCESSOR_BLOCK_LEVEL_REGEX: &str = "^<\\/?([^ >]+)";
/// upstream `inst_tp_UnescapeTreeprocessor` (flags `-`)
pub const RE_INST_TP_UNESCAPETREEPROCESSOR: &str = "\u{2}(\\d+)\u{3}";
/// upstream `inst_util_HTML_PLACEHOLDER_RE` (flags `-`)
pub const RE_INST_UTIL_HTML_PLACEHOLDER_RE: &str = "\u{2}wzxhzdk:([0-9]+)\u{3}";
/// upstream `inst_util_INLINE_PLACEHOLDER_RE` (flags `-`)
pub const RE_INST_UTIL_INLINE_PLACEHOLDER_RE: &str = "\u{2}klzzwxh:([0-9]+)\u{3}";
/// upstream `pp_RawHtmlPostprocessor_BLOCK_LEVEL_REGEX` (flags `-`)
pub const RE_PP_RAWHTMLPOSTPROCESSOR_BLOCK_LEVEL_REGEX: &str = "^<\\/?([^ >]+)";
/// upstream `ser_RE_AMP` (flags `i`)
pub const RE_SER_RE_AMP: &str = "(?i)&(?!(?:\\#[0-9]+|\\#x[0-9a-f]+|[0-9a-z]+);)";
/// upstream `smarty_00_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_00_SUBSTITUTETEXTPATTERN: &str = "(?s)(?<!-)---(?!-)";
/// upstream `smarty_01_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_01_SUBSTITUTETEXTPATTERN: &str = "(?s)(?<!-)--(?!-)";
/// upstream `smarty_02_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_02_SUBSTITUTETEXTPATTERN: &str = "(?s)^'(?=[!\"#\\$\\%'()*+,-.\\/:;<=>?\\@\\[\\\\\\]\\^_`{|}~]\\B)";
/// upstream `smarty_03_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_03_SUBSTITUTETEXTPATTERN: &str = "(?s)^\"(?=[!\"#\\$\\%'()*+,-.\\/:;<=>?\\@\\[\\\\\\]\\^_`{|}~]\\B)";
/// upstream `smarty_04_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_04_SUBSTITUTETEXTPATTERN: &str = "(?s)\"'(?=\\w)";
/// upstream `smarty_05_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_05_SUBSTITUTETEXTPATTERN: &str = "(?s)'\"(?=\\w)";
/// upstream `smarty_06_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_06_SUBSTITUTETEXTPATTERN: &str = "(?s)(?<=[^\\ \\t\\r\\n\\[\\{\\(\\-\\u0002\\u0003])\\'\"";
/// upstream `smarty_07_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_07_SUBSTITUTETEXTPATTERN: &str = "(?s)(?<=[^\\ \\t\\r\\n\\[\\{\\(\\-\\u0002\\u0003])\\\"'";
/// upstream `smarty_08_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_08_SUBSTITUTETEXTPATTERN: &str = "(?s)(?<!\\w)'(?=\\d{2}s)";
/// upstream `smarty_09_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_09_SUBSTITUTETEXTPATTERN: &str = "(?s)(\\s|&nbsp;|--|–|—|&[mn]dash;|&#8211;|&#8212;)'(?=\\w)";
/// upstream `smarty_10_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_10_SUBSTITUTETEXTPATTERN: &str = "(?s)(?<=[^\\ \\t\\r\\n\\[\\{\\(\\-\\u0002\\u0003])'(?!\\s|s\\b|\\d)";
/// upstream `smarty_11_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_11_SUBSTITUTETEXTPATTERN: &str = "(?s)'(\\s|s\\b)";
/// upstream `smarty_12_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_12_SUBSTITUTETEXTPATTERN: &str = "(?s)'";
/// upstream `smarty_13_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_13_SUBSTITUTETEXTPATTERN: &str = "(?s)(\\s|&nbsp;|--|–|—|&[mn]dash;|&#8211;|&#8212;)\"(?=\\w)";
/// upstream `smarty_14_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_14_SUBSTITUTETEXTPATTERN: &str = "(?s)\"(?=\\s)";
/// upstream `smarty_15_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_15_SUBSTITUTETEXTPATTERN: &str = "(?s)(?<=[^\\ \\t\\r\\n\\[\\{\\(\\-\\u0002\\u0003])\"";
/// upstream `smarty_16_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_16_SUBSTITUTETEXTPATTERN: &str = "(?s)\"";
/// upstream `smarty_17_SubstituteTextPattern` (flags `s`)
pub const RE_SMARTY_17_SUBSTITUTETEXTPATTERN: &str = "(?s)(?<!\\.)\\.{3}(?!\\.)";
/// upstream `tp_UnescapeTreeprocessor_RE` (flags `-`)
pub const RE_TP_UNESCAPETREEPROCESSOR_RE: &str = "\u{2}(\\d+)\u{3}";
/// upstream `util_HTML_PLACEHOLDER_RE` (flags `-`)
pub const RE_UTIL_HTML_PLACEHOLDER_RE: &str = "\u{2}wzxhzdk:([0-9]+)\u{3}";
/// upstream `util_INLINE_PLACEHOLDER_RE` (flags `-`)
pub const RE_UTIL_INLINE_PLACEHOLDER_RE: &str = "\u{2}klzzwxh:([0-9]+)\u{3}";
