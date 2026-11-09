//! Everything drawn by hand: a pale backdrop, rounded tiles with their own patch of night sky, capsule buttons, the curved
//! bar along the bottom with its two round buttons, and text placed by the shape of its letters. No pictures anywhere.

use eframe::egui::epaint::{CubicBezierShape, Mesh, PathShape, PathStroke, Vertex, WHITE_UV};
use eframe::egui::{Align, Color32, CornerRadius, FontId, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, Vec2};
use std::f32::consts::{FRAC_PI_2, TAU};

pub const INK: Color32 = Color32::from_rgb(70, 74, 82);
pub const SOFT: Color32 = Color32::from_rgb(128, 134, 144);
pub const BLUE: Color32 = Color32::from_rgb(52, 190, 240);
pub const BLUE_INK: Color32 = Color32::from_rgb(20, 126, 186);
pub const BLUE_EDGE: Color32 = Color32::from_rgb(150, 212, 240);
pub const RED: Color32 = Color32::from_rgb(208, 76, 76);
pub const GREEN: Color32 = Color32::from_rgb(40, 158, 100);
/// A state that waits for the person.
pub const AMBER: Color32 = Color32::from_rgb(196, 128, 24);
pub const GOLD: Color32 = Color32::from_rgb(255, 208, 40);
pub const LINE: Color32 = Color32::from_rgb(198, 204, 212);
const BACKDROP_TOP: Color32 = Color32::from_rgb(252, 253, 254);
const BACKDROP_BOTTOM: Color32 = Color32::from_rgb(232, 236, 240);
const BAR_TOP: Color32 = Color32::from_rgb(214, 218, 224);
const BAR_BOTTOM: Color32 = Color32::from_rgb(190, 195, 202);
const NIGHT_TOP: Color32 = Color32::from_rgb(8, 14, 62);
const NIGHT_BOTTOM: Color32 = Color32::from_rgb(38, 62, 150);

pub const BAR_HEIGHT: f32 = 96.0;
pub const ROUND_RADIUS: f32 = 46.0;
pub const BURST_SECONDS: f64 = 1.0;

const PLANETS: [Color32; 6] = [
    Color32::from_rgb(120, 200, 100),
    Color32::from_rgb(240, 150, 70),
    Color32::from_rgb(240, 120, 170),
    Color32::from_rgb(90, 170, 240),
    Color32::from_rgb(170, 130, 240),
    Color32::from_rgb(240, 200, 80),
];

/// A small, stable random number: the same stars every frame.
fn noise(seed: u32) -> f32 {
    let mut x = seed.wrapping_mul(2_654_435_761);
    x ^= x >> 15;
    x = x.wrapping_mul(2_246_822_519);
    x ^= x >> 13;
    (x & 0xFFFF) as f32 / 65535.0
}

/// Text placed by the middle of a capital letter, so every label sits level in what holds it, whatever its font's spacing
/// above and below. `at`: the point to line up with, by `align` across and by the letters' middle down.
pub fn text(painter: &Painter, at: Pos2, align: Align, string: &str, font: &FontId, color: Color32) {
    let capital = painter.layout_no_wrap("H".to_string(), font.clone(), color);
    let galley = painter.layout_no_wrap(string.to_string(), font.clone(), color);
    let x = match align {
        Align::Min => at.x,
        Align::Center => at.x - galley.size().x / 2.0,
        Align::Max => at.x - galley.size().x,
    };
    painter.galley(Pos2::new(x, at.y - capital.mesh_bounds.center().y), galley, color);
}

pub fn text_width(painter: &Painter, string: &str, font: &FontId) -> f32 {
    painter.layout_no_wrap(string.to_string(), font.clone(), INK).size().x
}

/// The text cut to a width, with dots if it had to be.
pub fn fitted(painter: &Painter, string: &str, font: &FontId, width: f32) -> String {
    if text_width(painter, string, font) <= width {
        return string.to_string();
    }
    let mut cut: Vec<char> = string.chars().collect();
    while !cut.is_empty() {
        cut.pop();
        let tried = format!("{}...", cut.iter().collect::<String>().trim_end());
        if text_width(painter, &tried, font) <= width {
            return tried;
        }
    }
    "...".to_string()
}

/// The words on at most two lines inside a width.
pub fn two_lines(painter: &Painter, string: &str, font: &FontId, width: f32) -> Vec<String> {
    let mut lines = vec![String::new()];
    let words: Vec<&str> = string.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        let last = lines.last().cloned().unwrap_or_default();
        let joined = if last.is_empty() { (*word).to_string() } else { format!("{last} {word}") };
        if last.is_empty() || text_width(painter, &joined, font) <= width {
            *lines.last_mut().unwrap() = joined;
        } else if lines.len() == 1 {
            lines.push((*word).to_string());
        } else {
            let rest = words[i..].join(" ");
            *lines.last_mut().unwrap() = format!("{last} {rest}");
            break;
        }
    }
    lines.iter().map(|line| fitted(painter, line, font, width)).collect()
}

fn vertical_gradient(painter: &Painter, area: Rect, top: Color32, bottom: Color32) {
    let mut mesh = Mesh::default();
    mesh.colored_vertex(area.left_top(), top);
    mesh.colored_vertex(area.right_top(), top);
    mesh.colored_vertex(area.left_bottom(), bottom);
    mesh.colored_vertex(area.right_bottom(), bottom);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(1, 2, 3);
    painter.add(Shape::mesh(mesh));
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let m = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t) as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

/// A rounded rectangle lit from above: the shape of every button and tile here.
fn glossy(painter: &Painter, area: Rect, radius: f32, top: Color32, bottom: Color32) {
    let inset_at = |y: f32| {
        let from_edge = y.min(area.height() - y);
        if from_edge >= radius {
            0.0
        } else {
            radius - (radius * radius - (radius - from_edge) * (radius - from_edge)).max(0.0).sqrt()
        }
    };
    let mut rows: Vec<f32> = (0..=14).map(|i| i as f32 / 14.0 * area.height()).collect();
    for i in 1..=10 {
        let step = radius * i as f32 / 10.0;
        rows.push(step.min(area.height()));
        rows.push((area.height() - step).max(0.0));
    }
    rows.sort_by(f32::total_cmp);
    rows.dedup();
    let mut mesh = Mesh::default();
    for (i, &y) in rows.iter().enumerate() {
        let inset = inset_at(y);
        let color = mix(top, bottom, y / area.height());
        mesh.colored_vertex(Pos2::new(area.left() + inset, area.top() + y), color);
        mesh.colored_vertex(Pos2::new(area.right() - inset, area.top() + y), color);
        if i != 0 {
            let at = (i as u32) * 2;
            mesh.add_triangle(at - 2, at - 1, at);
            mesh.add_triangle(at - 1, at, at + 1);
        }
    }
    painter.add(Shape::mesh(mesh));
}

/// The pale backdrop with its fine ruled lines.
pub fn backdrop(painter: &Painter, area: Rect) {
    vertical_gradient(painter, area, BACKDROP_TOP, BACKDROP_BOTTOM);
    let mut y = area.top() + 3.0;
    while y < area.bottom() {
        painter.line_segment([Pos2::new(area.left(), y), Pos2::new(area.right(), y)], Stroke::new(1.0, Color32::from_black_alpha(5)));
        y += 4.0;
    }
}

/// The grey bar along the bottom: it dips in the middle and rises at each end, around the two round buttons.
pub fn bottom_bar(painter: &Painter, area: Rect) {
    let top = area.bottom() - BAR_HEIGHT;
    let rise = 30.0;
    let shoulder = 150.0;
    let curve = |from: Pos2, c1: Pos2, c2: Pos2, to: Pos2| CubicBezierShape::from_points_stroke([from, c1, c2, to], false, Color32::TRANSPARENT, Stroke::NONE).flatten(Some(0.4));
    let left_end = Pos2::new(area.left(), top - rise);
    let left_bend = Pos2::new(area.left() + shoulder, top - rise * 0.15);
    let mut outline = curve(left_end, left_end + Vec2::new(70.0, 0.0), left_bend - Vec2::new(60.0, 0.0), left_bend + Vec2::new(40.0, 0.0));
    let right_end = Pos2::new(area.right(), top - rise);
    let right_bend = Pos2::new(area.right() - shoulder, top - rise * 0.15);
    outline.extend(curve(right_bend - Vec2::new(40.0, 0.0), right_bend + Vec2::new(60.0, 0.0), right_end - Vec2::new(70.0, 0.0), right_end));
    let mut mesh = Mesh::default();
    for (i, point) in outline.iter().enumerate() {
        let down = (area.bottom() - point.y) / (BAR_HEIGHT + rise);
        mesh.colored_vertex(*point, BAR_TOP);
        mesh.colored_vertex(Pos2::new(point.x, area.bottom()), mix(BAR_TOP, BAR_BOTTOM, down.clamp(0.0, 1.0)));
        if i != 0 {
            let at = (i as u32) * 2;
            mesh.add_triangle(at - 2, at - 1, at);
            mesh.add_triangle(at - 1, at, at + 1);
        }
    }
    painter.add(Shape::mesh(mesh));
    painter.add(PathShape::line(outline, PathStroke::new(3.0, BLUE)));
}

/// A tile with nothing in it.
pub fn empty_tile(painter: &Painter, area: Rect, hovered: bool) {
    glossy(painter, area, 16.0, Color32::from_rgb(250, 251, 252), Color32::from_rgb(226, 230, 235));
    painter.rect_stroke(area, CornerRadius::same(16), Stroke::new(1.5, LINE), StrokeKind::Inside);
    if hovered {
        tile_outline(painter, area);
    }
}

/// The thick blue line around whatever the pointer is on.
pub fn tile_outline(painter: &Painter, area: Rect) {
    painter.rect_stroke(area.expand(2.0), CornerRadius::same(18), Stroke::new(4.0, BLUE), StrokeKind::Middle);
}

/// A mod's tile: white, with its own patch of night sky above and its planet in it. Gives back the sky's area.
pub fn mod_tile(painter: &Painter, area: Rect, hue: u32, dim: bool, time: f64) -> Rect {
    glossy(painter, area, 16.0, Color32::WHITE, Color32::from_rgb(238, 241, 244));
    let sky = Rect::from_min_max(area.min + Vec2::splat(6.0), Pos2::new(area.right() - 6.0, area.top() + area.height() * 0.52));
    glossy(painter, sky, 11.0, if dim { Color32::from_rgb(70, 72, 84) } else { NIGHT_TOP }, if dim { Color32::from_rgb(110, 114, 130) } else { NIGHT_BOTTOM });
    for i in 0..14u32 {
        let seed = hue.wrapping_mul(13).wrapping_add(i * 3);
        let at = sky.min + Vec2::new(10.0 + noise(seed) * (sky.width() - 20.0), 8.0 + noise(seed + 1) * (sky.height() - 16.0));
        let twinkle = 0.5 + 0.5 * (time as f32 * (0.7 + noise(seed + 2)) + i as f32).sin();
        painter.circle_filled(at, 0.8 + noise(seed + 2) * 0.7, Color32::from_white_alpha((90.0 + 165.0 * twinkle) as u8));
    }
    // A small world, whole, resting in the middle of its sky.
    let radius = sky.height() * 0.3;
    let centre = sky.center();
    let color = if dim { Color32::from_rgb(140, 142, 150) } else { PLANETS[hue as usize % PLANETS.len()] };
    painter.circle_filled(centre, radius, mix(color, Color32::BLACK, 0.25));
    painter.circle_filled(centre - Vec2::splat(radius * 0.12), radius * 0.84, color);
    painter.circle_filled(centre - Vec2::splat(radius * 0.38), radius * 0.2, Color32::from_white_alpha(170));
    painter.rect_stroke(area, CornerRadius::same(16), Stroke::new(1.5, LINE), StrokeKind::Inside);
    sky
}

/// A white capsule with a blue edge. `on`: under the pointer.
pub fn capsule(painter: &Painter, area: Rect, label: &str, font: &FontId, on: bool, enabled: bool) {
    let radius = area.height() / 2.0;
    painter.rect_filled(area.translate(Vec2::new(0.0, 2.0)), CornerRadius::same(radius as u8), Color32::from_black_alpha(26));
    glossy(painter, area, radius, Color32::WHITE, Color32::from_rgb(222, 228, 234));
    let edge = if !enabled {
        LINE
    } else if on {
        BLUE
    } else {
        BLUE_EDGE
    };
    painter.rect_stroke(area, CornerRadius::same(radius as u8), Stroke::new(if on { 3.5 } else { 2.5 }, edge), StrokeKind::Inside);
    text(painter, area.center(), Align::Center, label, font, if enabled { BLUE_INK } else { SOFT });
}

/// One of the bar's two round buttons.
pub fn round_button(painter: &Painter, centre: Pos2, radius: f32, label: &str, font: &FontId, on: bool, enabled: bool) {
    painter.circle_filled(centre + Vec2::new(0.0, 2.0), radius + 1.0, Color32::from_black_alpha(36));
    let area = Rect::from_center_size(centre, Vec2::splat(radius * 2.0));
    glossy(painter, area, radius, Color32::WHITE, Color32::from_rgb(216, 222, 228));
    let edge = if !enabled {
        LINE
    } else if on {
        BLUE
    } else {
        BLUE_EDGE
    };
    painter.circle_stroke(centre, radius - 1.5, Stroke::new(if on { 4.0 } else { 3.0 }, edge));
    text(painter, centre, Align::Center, label, font, if enabled { BLUE_INK } else { SOFT });
}

/// A page arrow at the screen's edge.
pub fn page_arrow(painter: &Painter, centre: Pos2, left: bool, on: bool) {
    let d = if left { -1.0 } else { 1.0 };
    let color = if on { BLUE } else { BLUE_EDGE };
    let stroke = Stroke::new(5.0, color);
    let (tip, up, down) = (centre + Vec2::new(d * 7.0, 0.0), centre + Vec2::new(-d * 7.0, -14.0), centre + Vec2::new(-d * 7.0, 14.0));
    painter.line_segment([up, tip], stroke);
    painter.line_segment([down, tip], stroke);
    for at in [tip, up, down] {
        painter.circle_filled(at, 2.5, color);
    }
}

/// The circle a mod is ticked in: filled with a tick when it will be converted.
pub fn tick(painter: &Painter, centre: Pos2, on: bool, hovered: bool) {
    if on {
        painter.circle_filled(centre, 13.0, Color32::WHITE);
        painter.circle_filled(centre, 11.0, if hovered { BLUE } else { BLUE_INK });
        let stroke = Stroke::new(2.4, Color32::WHITE);
        let (a, b, c) = (centre + Vec2::new(-5.0, 0.5), centre + Vec2::new(-1.5, 4.0), centre + Vec2::new(5.0, -3.5));
        painter.line_segment([a, b], stroke);
        painter.line_segment([b, c], stroke);
        for at in [a, b, c] {
            painter.circle_filled(at, 1.2, Color32::WHITE);
        }
    } else {
        painter.circle_filled(centre, 13.0, Color32::from_white_alpha(235));
        painter.circle_stroke(centre, 10.5, Stroke::new(2.0, if hovered { BLUE } else { LINE }));
    }
}

/// A plump five-pointed star: a smaller star grown evenly on every side, which rounds its tips the way a pen would.
pub fn star(painter: &Painter, centre: Pos2, radius: f32, turn: f32, color: Color32) {
    let grow = radius * 0.17;
    let reach = radius - grow;
    let inner = reach * 0.48;
    let points: Vec<Pos2> = (0..10u32)
        .map(|i| {
            let distance = if i % 2 == 0 { reach } else { inner };
            centre + Vec2::angled(turn - FRAC_PI_2 + i as f32 / 10.0 * TAU) * distance
        })
        .collect();
    let mut mesh = Mesh::default();
    mesh.colored_vertex(centre, color);
    for (i, at) in points.iter().enumerate() {
        mesh.vertices.push(Vertex { pos: *at, uv: WHITE_UV, color });
        let i = i as u32;
        mesh.add_triangle(0, i + 1, (i + 1) % 10 + 1);
    }
    painter.add(Shape::mesh(mesh));
    for i in 0..10 {
        painter.line_segment([points[i], points[(i + 1) % 10]], Stroke::new(grow * 2.0, color));
        if i % 2 == 0 {
            painter.circle_filled(points[i], grow, color);
        }
    }
}

/// A tiny star circling a tile's planet while its mod is worked on.
pub fn orbiting_star(painter: &Painter, centre: Pos2, reach: f32, time: f64) {
    let a = time as f32 * 3.2;
    star(painter, centre + Vec2::new(a.cos() * reach, a.sin() * reach * 0.5), 6.0, a, GOLD);
}

/// Star bits flying out of a tile as its mod finishes.
pub fn burst(painter: &Painter, centre: Pos2, age: f64) {
    let progress = (age / BURST_SECONDS) as f32;
    let fade = ((1.0 - progress) * 255.0) as u8;
    for i in 0..10u32 {
        let angle = i as f32 / 10.0 * TAU + noise(i + 40);
        let at = centre + Vec2::angled(angle) * (12.0 + progress * 60.0) * (0.7 + 0.4 * noise(i + 60));
        let tint = if i % 2 == 0 { GOLD } else { BLUE };
        star(painter, at, 3.0 + 3.0 * (1.0 - progress), angle + progress * 4.0, Color32::from_rgba_unmultiplied(tint.r(), tint.g(), tint.b(), fade));
    }
}
