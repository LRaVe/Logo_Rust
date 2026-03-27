use std::fs;
use svg_fmt::{line_segment, red, BeginSvg, EndSvg};

fn main() {
    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    svg.push_str(&format!("{}\n", BeginSvg { w: 300.0, h: 200.0 }));

    let lines = [
        line_segment(100.0, 100.0, 200.0, 100.0).color(red()).width(2.0),
        line_segment(200.0, 100.0, 200.0, 200.0).color(red()).width(2.0),
        line_segment(200.0, 200.0, 100.0, 200.0).color(red()).width(2.0),
        line_segment(100.0, 200.0, 100.0, 100.0).color(red()).width(2.0),
    ];

    for line in lines {
        svg.push_str("    ");
        svg.push_str(&format!("{}\n", line));
    }

    svg.push_str(&format!("{}\n", EndSvg));

    if let Err(e) = fs::write("demo_square.svg", svg) {
        eprintln!("Cannot write demo_square.svg: {}", e);
        return;
    }

    println!("demo_square.svg generated successfully");
}
