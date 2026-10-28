//! Built-in extensions (`markdown/extensions/*`).
//!
//! Each module mirrors one upstream extension file; the pipeline wires
//! stages by spec name. Unimplemented names decline in `pipeline`.

pub mod abbr;
pub mod admonition;
pub mod attr_list;
pub mod attr_list_tree;
pub mod meta;
pub mod codehilite;
pub mod def_list;
pub mod fenced_code;
pub mod footnotes;
pub mod legacy_attrs;
pub mod legacy_em;
pub mod md_in_html;
pub mod nl2br;
pub mod sane_lists;
pub mod smarty;
pub mod tables;
pub mod toc;
pub mod wikilinks;
