//! `Layout::with_subtitle` renders a second line centred under the title at a smaller,
//! muted size, and the title block reserves the extra height.

use kuva::backend::svg::SvgBackend;
use kuva::plot::scatter::ScatterPlot;
use kuva::render::layout::Layout;
use kuva::render::render::render_scatter;

fn scatter_svg(layout: Layout) -> String {
    let plot = ScatterPlot::new().with_data(vec![(1.0, 5.0), (4.5, 3.5), (5.0, 8.7)]);
    SvgBackend.render_scene(&render_scatter(&plot, layout).with_background(Some("white")))
}

/// Read a numeric attribute from the root `<svg>` element.
fn svg_root_attr(svg: &str, name: &str) -> f64 {
    let head = &svg[..svg.find('>').unwrap()];
    let key = format!("{name}=\"");
    let s = head.find(&key).unwrap() + key.len();
    let e = head[s..].find('"').unwrap() + s;
    head[s..e].parse().unwrap()
}

/// Return the full `<text …>content</text>` tag whose content is exactly `content`.
fn text_tag(svg: &str, content: &str) -> String {
    let needle = format!(">{content}</text>");
    let close = svg.find(&needle).unwrap_or_else(|| panic!("no <text> {content:?}"));
    let open = svg[..close].rfind("<text ").unwrap();
    svg[open..close + needle.len()].to_string()
}

fn base_layout() -> Layout {
    Layout::new((0.0, 10.0), (0.0, 10.0)).with_title("Main Title")
}

#[test]
fn subtitle_text_is_rendered() {
    let svg = scatter_svg(base_layout().with_subtitle("n = 1,234 cells"));
    std::fs::write("test_outputs/subtitle_basic.svg", &svg).unwrap();
    assert!(svg.contains(">n = 1,234 cells</text>"), "subtitle text should be drawn");
}

#[test]
fn subtitle_is_smaller_and_muted() {
    let svg = scatter_svg(base_layout().with_subtitle("a subtitle"));
    let title = text_tag(&svg, "Main Title");
    let subtitle = text_tag(&svg, "a subtitle");

    // Default title size is 18 → subtitle is round(18 * 0.7) = 13.
    assert!(title.contains("font-size=\"18\""), "title at default 18: {title}");
    assert!(subtitle.contains("font-size=\"13\""), "subtitle at 0.7x = 13: {subtitle}");
    // Subtitle is muted; the title inherits the default fill (no per-element fill).
    assert!(subtitle.contains("fill=\"#666666\""), "subtitle should be muted: {subtitle}");
    assert!(!title.contains("fill="), "title should not set a per-element fill: {title}");
    // Both are centre-anchored on the same x.
    assert!(subtitle.contains("text-anchor=\"middle\""));
}

#[test]
fn subtitle_reserves_extra_top_margin() {
    let without = scatter_svg(base_layout());
    let with = scatter_svg(base_layout().with_subtitle("a subtitle"));
    assert!(
        svg_root_attr(&with, "height") > svg_root_attr(&without, "height"),
        "subtitle must add top-margin height ({} !> {})",
        svg_root_attr(&with, "height"),
        svg_root_attr(&without, "height"),
    );
}

#[test]
fn subtitle_sits_below_title() {
    let svg = scatter_svg(base_layout().with_subtitle("a subtitle"));
    let y_of = |tag: &str| -> f64 {
        let s = tag.find("y=\"").unwrap() + 3;
        let e = tag[s..].find('"').unwrap() + s;
        tag[s..e].parse().unwrap()
    };
    assert!(
        y_of(&text_tag(&svg, "a subtitle")) > y_of(&text_tag(&svg, "Main Title")),
        "subtitle baseline should be below the title baseline"
    );
}
