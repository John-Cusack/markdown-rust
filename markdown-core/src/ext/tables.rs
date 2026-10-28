//! Port of `markdown/extensions/tables.py` (`table` block processor, priority 75).
//!
//! `TableProcessor` carries per-test state (`border`, `separator`) that
//! `run` consumes; the render-local copy lives on `Context`
//! (`table_border`, `table_separator`), mirroring the instance attributes.

use fancy_regex::Regex as F;

use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};
use crate::patterns_gen as P;

const PIPE_NONE: u8 = 0;
const PIPE_LEFT: u8 = 1;
const PIPE_RIGHT: u8 = 2;

fn re_code_pipes() -> F {
    compile(P::RE_EXT_TABLES_TABLEPROCESSOR_RE_CODE_PIPES)
}

fn re_end_border() -> F {
    compile(P::RE_EXT_TABLES_TABLEPROCESSOR_RE_END_BORDER)
}

fn strip_spaces(s: &str) -> &str {
    s.trim_matches(' ')
}

/// `TableProcessor.test` (also records `border`/`separator` on `ctx`).
pub fn test(ctx: &mut Context, block: &str) -> bool {
    let rows: Vec<&str> = block.split('\n').map(strip_spaces).collect();
    if rows.len() <= 1 {
        return false;
    }
    let header0 = rows[0];
    let mut border = PIPE_NONE;
    if header0.starts_with('|') {
        border |= PIPE_LEFT;
    }
    if re_end_border().find(header0).ok().flatten().is_some() {
        border |= PIPE_RIGHT;
    }
    let row = split_row(header0, border);
    let row0_len = row.len();
    let mut is_table = row0_len > 1;
    if !is_table && row0_len == 1 && border != PIPE_NONE {
        is_table = true;
        for r in rows.iter().skip(1) {
            is_table = r.starts_with('|');
            if !is_table {
                is_table = re_end_border().find(r).ok().flatten().is_some();
            }
            if !is_table {
                break;
            }
        }
    }
    if is_table {
        let row = split_row(rows[1], border);
        is_table = row.len() == row0_len
            && row.join("").chars().all(|c| c == '|' || c == ':' || c == '-' || c == ' ');
        if is_table {
            ctx.table_border = border;
            ctx.table_separator = row;
        }
    }
    is_table
}

/// `TableProcessor.run`.
pub fn run(ctx: &mut Context, parent: &mut Element, blocks: &mut Vec<String>) -> bool {
    if !test(ctx, &blocks[0].clone()) {
        return false;
    }
    let block = blocks.remove(0);
    let lines: Vec<&str> = block.split('\n').collect();
    let header = strip_spaces(lines[0]);
    let rows: Vec<&str> = if lines.len() < 3 { Vec::new() } else { lines[2..].to_vec() };
    let separator = ctx.table_separator.clone();
    let border = ctx.table_border;
    let _ = border;
    let mut align: Vec<Option<&str>> = Vec::new();
    for c in &separator {
        let c = strip_spaces(c);
        if c.starts_with(':') && c.ends_with(':') {
            align.push(Some("center"));
        } else if c.starts_with(':') {
            align.push(Some("left"));
        } else if c.ends_with(':') {
            align.push(Some("right"));
        } else {
            align.push(None);
        }
    }
    let use_align = ctx.ext_bool("tables", "use_align_attribute", false);
    let mut table = Element::new("table");
    let mut thead = Element::new("thead");
    build_row(&mut thead, header, &align, border, use_align);
    table.children.push(thead);
    let mut tbody = Element::new("tbody");
    if rows.is_empty() {
        build_empty_row(&mut tbody, align.len());
    } else {
        for row in rows {
            build_row(&mut tbody, strip_spaces(row), &align, border, use_align);
        }
    }
    table.children.push(tbody);
    parent.children.push(table);
    true
}

fn build_empty_row(parent: &mut Element, count: usize) {
    let mut tr = Element::new("tr");
    for _ in 0..count {
        tr.children.push(Element::new("td"));
    }
    parent.children.push(tr);
}

fn build_row(
    parent: &mut Element,
    row: &str,
    align: &[Option<&str>],
    border: u8,
    use_align: bool,
) {
    let tag = if parent.tag == "thead" { "th" } else { "td" };
    let mut tr = Element::new("tr");
    let cells = split_row(row, border);
    for (i, a) in align.iter().enumerate() {
        let mut c = Element::new(tag);
        c.text = Some(Text::Normal(
            cells.get(i).map(|s| strip_spaces(s).to_string()).unwrap_or_default(),
        ));
        if let Some(a) = a {
            if use_align {
                c.set_attr("align", a);
            } else {
                c.set_attr("style", &format!("text-align: {a};"));
            }
        }
        tr.children.push(c);
    }
    parent.children.push(tr);
}

fn split_row(row: &str, border: u8) -> Vec<String> {
    let mut row = row;
    let owned;
    if border != PIPE_NONE {
        let mut r = row;
        if let Some(stripped) = r.strip_prefix('|') {
            r = stripped;
        }
        owned = re_end_border().replace_all(r, "").into_owned();
        row = &owned;
    }
    split(row)
}

fn split(row: &str) -> Vec<String> {
    let re = re_code_pipes();
    let mut pipes: Vec<usize> = Vec::new();
    let mut tics: Vec<usize> = Vec::new();
    let mut tic_points: Vec<(usize, usize, usize)> = Vec::new();
    for m in re.captures_iter(row).filter_map(|r| r.ok()) {
        let whole = m.get(0).unwrap();
        let _ = whole;
        if let Some(g) = m.get(2) {
            tics.push(g.as_str().len() - 1);
            tic_points.push((g.start(), g.end() - 1, 1));
        } else if let Some(g) = m.get(3) {
            tics.push(g.as_str().len());
            tic_points.push((g.start(), g.end() - 1, 0));
        }
        if let Some(g) = m.get(5) {
            pipes.push(g.start());
        }
    }
    // Pair up tics.
    let mut tic_region: Vec<(usize, usize)> = Vec::new();
    let mut pos = 0usize;
    let tic_len = tics.len();
    while pos < tic_len {
        let tic_size = match tics[pos].checked_sub(tic_points[pos].2) {
            Some(0) | None => {
                pos += 1;
                continue;
            }
            Some(n) => n,
        };
        if let Some(index) = tics[pos + 1..].iter().position(|&t| t == tic_size) {
            let index = index + 1;
            tic_region.push((tic_points[pos].0, tic_points[pos + index].1));
            pos += index + 1;
        } else {
            pos += 1;
        }
    }
    // Resolve pipes.
    let mut good: Vec<usize> = Vec::new();
    for pipe in pipes {
        let mut throw_out = false;
        for region in &tic_region {
            if pipe < region.0 {
                break;
            } else if region.0 <= pipe && pipe <= region.1 {
                throw_out = true;
                break;
            }
        }
        if !throw_out {
            good.push(pipe);
        }
    }
    let mut elements = Vec::new();
    let mut p = 0usize;
    for pipe in good {
        elements.push(row[p..pipe].to_string());
        p = pipe + 1;
    }
    elements.push(row[p..].to_string());
    elements
}
