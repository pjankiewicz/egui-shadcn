//! Paints a Lucide icon using the egui `Painter`.

/// Paint a Lucide icon into `rect` with the given `color`.
///
/// The icon is rendered as vector strokes (not rasterised), matching the
/// Lucide default: stroke-width 2, round linecap/linejoin, 24×24 viewBox.
pub fn paint_icon(
    painter: &egui::Painter,
    rect: egui::Rect,
    icon: &super::lucide_icon::LucideIcon,
    color: egui::Color32,
) {
    paint_icon_svg(painter, rect, icon.svg_data(), color);
}

/// Paint a raw Lucide-style SVG body (the inner `<path>`/`<circle>`/… elements,
/// 24×24 viewBox, no `<svg>` wrapper) into `rect` with the given `color`.
///
/// Use this for custom icons authored in the Lucide grammar that are not part of
/// the [`LucideIcon`](super::lucide_icon::LucideIcon) set.
pub fn paint_icon_svg(
    painter: &egui::Painter,
    rect: egui::Rect,
    svg_body: &str,
    color: egui::Color32,
) {
    let elements = super::parse_svg::parse_svg(svg_body);
    let scale = rect.width().min(rect.height()) / 24.0;
    let xf = egui::emath::TSTransform::new(rect.min.to_vec2(), scale);
    let stroke = egui::Stroke::new(2.0 * scale, color);

    for element in &elements {
        paint_element(painter, element, xf, stroke);
    }
}

fn paint_element(
    painter: &egui::Painter,
    element: &super::icon_element::IconElement,
    xf: egui::emath::TSTransform,
    stroke: egui::Stroke,
) {
    match element {
        super::icon_element::IconElement::Path(commands) => {
            paint_path(painter, commands, xf, stroke);
        }
        super::icon_element::IconElement::Circle { cx, cy, r } => {
            painter.circle_stroke(map_point(xf, *cx, *cy), r * xf.scaling, stroke);
        }
        super::icon_element::IconElement::Rect {
            x,
            y,
            width,
            height,
            rx,
        } => {
            let r = egui::Rect::from_min_size(
                map_point(xf, *x, *y),
                egui::vec2(width * xf.scaling, height * xf.scaling),
            );
            let cr = egui::CornerRadius::same((rx * xf.scaling) as u8);
            painter.rect_stroke(r, cr, stroke, egui::epaint::StrokeKind::Outside);
        }
        super::icon_element::IconElement::Line { x1, y1, x2, y2 } => {
            painter.line_segment([map_point(xf, *x1, *y1), map_point(xf, *x2, *y2)], stroke);
        }
        super::icon_element::IconElement::Polyline(points) => {
            let pts: Vec<egui::Pos2> = points.iter().map(|(x, y)| map_point(xf, *x, *y)).collect();
            if pts.len() >= 2 {
                painter.add(egui::Shape::line(pts, stroke));
            }
        }
        super::icon_element::IconElement::Polygon(points) => {
            let pts: Vec<egui::Pos2> = points.iter().map(|(x, y)| map_point(xf, *x, *y)).collect();
            if pts.len() >= 2 {
                painter.add(egui::Shape::closed_line(pts, stroke));
            }
        }
        super::icon_element::IconElement::Ellipse { cx, cy, rx, ry } => {
            paint_ellipse(
                painter,
                egui::pos2(*cx, *cy),
                egui::vec2(*rx, *ry),
                xf,
                stroke,
            );
        }
    }
}

// ── Path tessellation ───────────────────────────────────────────

fn paint_path(
    painter: &egui::Painter,
    commands: &[super::path_command::PathCommand],
    xf: egui::emath::TSTransform,
    stroke: egui::Stroke,
) {
    let mut points: Vec<egui::Pos2> = Vec::new();
    let mut cx: f32 = 0.0;
    let mut cy: f32 = 0.0;
    let mut subpath_start_x: f32 = 0.0;
    let mut subpath_start_y: f32 = 0.0;
    // For smooth cubic/quad continuations
    let mut last_ctrl_x: f32 = 0.0;
    let mut last_ctrl_y: f32 = 0.0;
    let mut last_was_cubic = false;
    let mut last_was_quad = false;

    for cmd in commands {
        match cmd {
            super::path_command::PathCommand::MoveToAbs(x, y) => {
                flush_subpath(painter, &mut points, stroke, false);
                cx = *x;
                cy = *y;
                subpath_start_x = cx;
                subpath_start_y = cy;
                points.push(map_point(xf, cx, cy));
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::MoveToRel(dx, dy) => {
                flush_subpath(painter, &mut points, stroke, false);
                cx += dx;
                cy += dy;
                subpath_start_x = cx;
                subpath_start_y = cy;
                points.push(map_point(xf, cx, cy));
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::LineToAbs(x, y) => {
                cx = *x;
                cy = *y;
                points.push(map_point(xf, cx, cy));
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::LineToRel(dx, dy) => {
                cx += dx;
                cy += dy;
                points.push(map_point(xf, cx, cy));
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::HorizontalAbs(x) => {
                cx = *x;
                points.push(map_point(xf, cx, cy));
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::HorizontalRel(dx) => {
                cx += dx;
                points.push(map_point(xf, cx, cy));
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::VerticalAbs(y) => {
                cy = *y;
                points.push(map_point(xf, cx, cy));
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::VerticalRel(dy) => {
                cy += dy;
                points.push(map_point(xf, cx, cy));
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::CubicAbs(x1, y1, x2, y2, x, y) => {
                tessellate_cubic(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(*x1, *y1),
                    egui::pos2(*x2, *y2),
                    egui::pos2(*x, *y),
                    xf,
                );
                last_ctrl_x = *x2;
                last_ctrl_y = *y2;
                cx = *x;
                cy = *y;
                last_was_cubic = true;
                last_was_quad = false;
            }
            super::path_command::PathCommand::CubicRel(dx1, dy1, dx2, dy2, dx, dy) => {
                let (x1, y1) = (cx + dx1, cy + dy1);
                let (x2, y2) = (cx + dx2, cy + dy2);
                let (ex, ey) = (cx + dx, cy + dy);
                tessellate_cubic(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(x1, y1),
                    egui::pos2(x2, y2),
                    egui::pos2(ex, ey),
                    xf,
                );
                last_ctrl_x = x2;
                last_ctrl_y = y2;
                cx = ex;
                cy = ey;
                last_was_cubic = true;
                last_was_quad = false;
            }
            super::path_command::PathCommand::SmoothCubicAbs(x2, y2, x, y) => {
                let (x1, y1) = if last_was_cubic {
                    (2.0 * cx - last_ctrl_x, 2.0 * cy - last_ctrl_y)
                } else {
                    (cx, cy)
                };
                tessellate_cubic(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(x1, y1),
                    egui::pos2(*x2, *y2),
                    egui::pos2(*x, *y),
                    xf,
                );
                last_ctrl_x = *x2;
                last_ctrl_y = *y2;
                cx = *x;
                cy = *y;
                last_was_cubic = true;
                last_was_quad = false;
            }
            super::path_command::PathCommand::SmoothCubicRel(dx2, dy2, dx, dy) => {
                let (x1, y1) = if last_was_cubic {
                    (2.0 * cx - last_ctrl_x, 2.0 * cy - last_ctrl_y)
                } else {
                    (cx, cy)
                };
                let (x2, y2) = (cx + dx2, cy + dy2);
                let (ex, ey) = (cx + dx, cy + dy);
                tessellate_cubic(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(x1, y1),
                    egui::pos2(x2, y2),
                    egui::pos2(ex, ey),
                    xf,
                );
                last_ctrl_x = x2;
                last_ctrl_y = y2;
                cx = ex;
                cy = ey;
                last_was_cubic = true;
                last_was_quad = false;
            }
            super::path_command::PathCommand::QuadAbs(x1, y1, x, y) => {
                tessellate_quad(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(*x1, *y1),
                    egui::pos2(*x, *y),
                    xf,
                );
                last_ctrl_x = *x1;
                last_ctrl_y = *y1;
                cx = *x;
                cy = *y;
                last_was_quad = true;
                last_was_cubic = false;
            }
            super::path_command::PathCommand::QuadRel(dx1, dy1, dx, dy) => {
                let (x1, y1) = (cx + dx1, cy + dy1);
                let (ex, ey) = (cx + dx, cy + dy);
                tessellate_quad(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(x1, y1),
                    egui::pos2(ex, ey),
                    xf,
                );
                last_ctrl_x = x1;
                last_ctrl_y = y1;
                cx = ex;
                cy = ey;
                last_was_quad = true;
                last_was_cubic = false;
            }
            super::path_command::PathCommand::SmoothQuadAbs(x, y) => {
                let (x1, y1) = if last_was_quad {
                    (2.0 * cx - last_ctrl_x, 2.0 * cy - last_ctrl_y)
                } else {
                    (cx, cy)
                };
                tessellate_quad(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(x1, y1),
                    egui::pos2(*x, *y),
                    xf,
                );
                last_ctrl_x = x1;
                last_ctrl_y = y1;
                cx = *x;
                cy = *y;
                last_was_quad = true;
                last_was_cubic = false;
            }
            super::path_command::PathCommand::SmoothQuadRel(dx, dy) => {
                let (x1, y1) = if last_was_quad {
                    (2.0 * cx - last_ctrl_x, 2.0 * cy - last_ctrl_y)
                } else {
                    (cx, cy)
                };
                let (ex, ey) = (cx + dx, cy + dy);
                tessellate_quad(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(x1, y1),
                    egui::pos2(ex, ey),
                    xf,
                );
                last_ctrl_x = x1;
                last_ctrl_y = y1;
                cx = ex;
                cy = ey;
                last_was_quad = true;
                last_was_cubic = false;
            }
            super::path_command::PathCommand::ArcAbs {
                rx,
                ry,
                angle,
                large_arc,
                sweep,
                x,
                y,
            } => {
                tessellate_arc(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(*x, *y),
                    ArcParams {
                        radii: egui::vec2(*rx, *ry),
                        x_rotation_deg: *angle,
                        large_arc: *large_arc,
                        sweep: *sweep,
                    },
                    xf,
                );
                cx = *x;
                cy = *y;
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::ArcRel {
                rx,
                ry,
                angle,
                large_arc,
                sweep,
                x,
                y,
            } => {
                let (ex, ey) = (cx + x, cy + y);
                tessellate_arc(
                    &mut points,
                    egui::pos2(cx, cy),
                    egui::pos2(ex, ey),
                    ArcParams {
                        radii: egui::vec2(*rx, *ry),
                        x_rotation_deg: *angle,
                        large_arc: *large_arc,
                        sweep: *sweep,
                    },
                    xf,
                );
                cx = ex;
                cy = ey;
                last_was_cubic = false;
                last_was_quad = false;
            }
            super::path_command::PathCommand::Close => {
                cx = subpath_start_x;
                cy = subpath_start_y;
                flush_subpath(painter, &mut points, stroke, true);
            }
        }
    }

    flush_subpath(painter, &mut points, stroke, false);
}

/// Maps a point from the icon's 24×24 user space into screen space.
fn map_point(xf: egui::emath::TSTransform, x: f32, y: f32) -> egui::Pos2 {
    xf * egui::pos2(x, y)
}

fn flush_subpath(
    painter: &egui::Painter,
    points: &mut Vec<egui::Pos2>,
    stroke: egui::Stroke,
    closed: bool,
) {
    if points.len() >= 2 {
        let pts = std::mem::take(points);
        if closed {
            painter.add(egui::Shape::closed_line(pts, stroke));
        } else {
            painter.add(egui::Shape::line(pts, stroke));
        }
    } else {
        points.clear();
    }
}

// ── Bézier tessellation ─────────────────────────────────────────

const CUBIC_SEGMENTS: usize = 8;
const QUAD_SEGMENTS: usize = 6;

fn tessellate_cubic(
    points: &mut Vec<egui::Pos2>,
    p0: egui::Pos2,
    p1: egui::Pos2,
    p2: egui::Pos2,
    p3: egui::Pos2,
    xf: egui::emath::TSTransform,
) {
    for i in 1..=CUBIC_SEGMENTS {
        let t = i as f32 / CUBIC_SEGMENTS as f32;
        let u = 1.0 - t;
        let x = u * u * u * p0.x + 3.0 * u * u * t * p1.x + 3.0 * u * t * t * p2.x + t * t * t * p3.x;
        let y = u * u * u * p0.y + 3.0 * u * u * t * p1.y + 3.0 * u * t * t * p2.y + t * t * t * p3.y;
        points.push(map_point(xf, x, y));
    }
}

fn tessellate_quad(
    points: &mut Vec<egui::Pos2>,
    p0: egui::Pos2,
    p1: egui::Pos2,
    p2: egui::Pos2,
    xf: egui::emath::TSTransform,
) {
    for i in 1..=QUAD_SEGMENTS {
        let t = i as f32 / QUAD_SEGMENTS as f32;
        let u = 1.0 - t;
        let x = u * u * p0.x + 2.0 * u * t * p1.x + t * t * p2.x;
        let y = u * u * p0.y + 2.0 * u * t * p1.y + t * t * p2.y;
        points.push(map_point(xf, x, y));
    }
}

// ── Arc to Bézier ───────────────────────────────────────────────

/// Convert an SVG arc to one or more cubic Bézier segments.
///
/// Follows the SVG spec endpoint-to-center parameterisation.
/// The radii and flags of an SVG `A`/`a` path command.
#[derive(Clone, Copy)]
struct ArcParams {
    radii: egui::Vec2,
    x_rotation_deg: f32,
    large_arc: bool,
    sweep: bool,
}

/// The rotated ellipse an arc sweeps around, in the icon's user space.
#[derive(Clone, Copy)]
struct ArcBasis {
    center: egui::Pos2,
    radii: egui::Vec2,
    sin_phi: f32,
    cos_phi: f32,
}

fn tessellate_arc(
    points: &mut Vec<egui::Pos2>,
    from: egui::Pos2,
    to: egui::Pos2,
    arc: ArcParams,
    xf: egui::emath::TSTransform,
) {
    let (x1, y1) = (from.x, from.y);
    let (x2, y2) = (to.x, to.y);
    let ArcParams {
        x_rotation_deg,
        large_arc,
        sweep,
        ..
    } = arc;
    let (mut rx, mut ry) = (arc.radii.x, arc.radii.y);

    // Degenerate: zero radius → line
    if rx.abs() < 1e-6 || ry.abs() < 1e-6 {
        points.push(map_point(xf, x2, y2));
        return;
    }

    rx = rx.abs();
    ry = ry.abs();
    let phi = x_rotation_deg.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();

    // Step 1: compute (x1', y1') in rotated coordinate system
    let dx2 = (x1 - x2) / 2.0;
    let dy2 = (y1 - y2) / 2.0;
    let x1p = cos_phi * dx2 + sin_phi * dy2;
    let y1p = -sin_phi * dx2 + cos_phi * dy2;

    // Step 2: correct radii if too small
    let x1p2 = x1p * x1p;
    let y1p2 = y1p * y1p;
    let rx2 = rx * rx;
    let ry2 = ry * ry;
    let lambda = x1p2 / rx2 + y1p2 / ry2;
    if lambda > 1.0 {
        let sqrt_lambda = lambda.sqrt();
        rx *= sqrt_lambda;
        ry *= sqrt_lambda;
    }
    let rx2 = rx * rx;
    let ry2 = ry * ry;

    // Step 3: compute center point (cx', cy')
    let num = (rx2 * ry2 - rx2 * y1p2 - ry2 * x1p2).max(0.0);
    let den = rx2 * y1p2 + ry2 * x1p2;
    let sq = if den > 0.0 { (num / den).sqrt() } else { 0.0 };
    let sign = if large_arc == sweep { -1.0 } else { 1.0 };
    let cxp = sign * sq * (rx * y1p / ry);
    let cyp = sign * sq * (-(ry * x1p / rx));

    // Step 4: compute center (cx, cy) in original coordinates
    let cx = cos_phi * cxp - sin_phi * cyp + (x1 + x2) / 2.0;
    let cy = sin_phi * cxp + cos_phi * cyp + (y1 + y2) / 2.0;

    // Step 5: compute start angle and sweep angle
    let theta1 = angle_between(
        egui::vec2(1.0, 0.0),
        egui::vec2((x1p - cxp) / rx, (y1p - cyp) / ry),
    );
    let mut dtheta = angle_between(
        egui::vec2((x1p - cxp) / rx, (y1p - cyp) / ry),
        egui::vec2((-x1p - cxp) / rx, (-y1p - cyp) / ry),
    );

    if !sweep && dtheta > 0.0 {
        dtheta -= std::f32::consts::TAU;
    } else if sweep && dtheta < 0.0 {
        dtheta += std::f32::consts::TAU;
    }

    // Step 6: approximate arc with cubic Bézier segments
    let n_segs = ((dtheta.abs() / (std::f32::consts::FRAC_PI_4)).ceil() as usize).max(1);
    let seg_angle = dtheta / n_segs as f32;

    let basis = ArcBasis {
        center: egui::pos2(cx, cy),
        radii: egui::vec2(rx, ry),
        sin_phi,
        cos_phi,
    };
    for i in 0..n_segs {
        let a1 = theta1 + seg_angle * i as f32;
        let a2 = a1 + seg_angle;
        arc_segment_to_cubic(points, basis, a1, a2, xf);
    }
}

fn arc_segment_to_cubic(
    points: &mut Vec<egui::Pos2>,
    basis: ArcBasis,
    a1: f32,
    a2: f32,
    xf: egui::emath::TSTransform,
) {
    let ArcBasis {
        center,
        radii,
        sin_phi,
        cos_phi,
    } = basis;
    let (cx, cy) = (center.x, center.y);
    let (rx, ry) = (radii.x, radii.y);

    let half = (a2 - a1) / 2.0;
    let alpha = half.sin() * ((4.0 + 3.0 * (2.0 * half).tan().powi(2)).sqrt() - 1.0) / 3.0;

    let (sin1, cos1) = a1.sin_cos();
    let (sin2, cos2) = a2.sin_cos();

    let ex1 = rx * cos1;
    let ey1 = ry * sin1;
    let ex2 = rx * cos2;
    let ey2 = ry * sin2;

    // Control point 1
    let dx1 = -rx * sin1;
    let dy1 = ry * cos1;
    let cp1x = cx + cos_phi * (ex1 + alpha * dx1) - sin_phi * (ey1 + alpha * dy1);
    let cp1y = cy + sin_phi * (ex1 + alpha * dx1) + cos_phi * (ey1 + alpha * dy1);

    // Control point 2
    let dx2 = -rx * sin2;
    let dy2 = ry * cos2;
    let cp2x = cx + cos_phi * (ex2 - alpha * dx2) - sin_phi * (ey2 - alpha * dy2);
    let cp2y = cy + sin_phi * (ex2 - alpha * dx2) + cos_phi * (ey2 - alpha * dy2);

    // End point
    let px = cx + cos_phi * ex2 - sin_phi * ey2;
    let py = cy + sin_phi * ex2 + cos_phi * ey2;

    // Tessellate this cubic segment
    let prev_x = cx + cos_phi * ex1 - sin_phi * ey1;
    let prev_y = cy + sin_phi * ex1 + cos_phi * ey1;
    tessellate_cubic(
        points,
        egui::pos2(prev_x, prev_y),
        egui::pos2(cp1x, cp1y),
        egui::pos2(cp2x, cp2y),
        egui::pos2(px, py),
        xf,
    );
}

fn angle_between(u: egui::Vec2, v: egui::Vec2) -> f32 {
    let (ux, uy, vx, vy) = (u.x, u.y, v.x, v.y);
    let dot = ux * vx + uy * vy;
    let len = (ux * ux + uy * uy).sqrt() * (vx * vx + vy * vy).sqrt();
    let cos_val = (dot / len).clamp(-1.0, 1.0);
    let angle = cos_val.acos();
    if ux * vy - uy * vx < 0.0 {
        -angle
    } else {
        angle
    }
}

// ── Ellipse approximation ───────────────────────────────────────

fn paint_ellipse(
    painter: &egui::Painter,
    center: egui::Pos2,
    radii: egui::Vec2,
    xf: egui::emath::TSTransform,
    stroke: egui::Stroke,
) {
    const N: usize = 32;
    let pts: Vec<egui::Pos2> = (0..N)
        .map(|i| {
            let angle = std::f32::consts::TAU * i as f32 / N as f32;
            let x = center.x + radii.x * angle.cos();
            let y = center.y + radii.y * angle.sin();
            map_point(xf, x, y)
        })
        .collect();

    painter.add(egui::Shape::closed_line(pts, stroke));
}

#[cfg(test)]
mod tests {
    //! Characterization tests for the Bézier tessellation helpers.
    //!
    //! `parse_path` and `parse_svg` are covered by their own tests, but they
    //! stop at the `PathCommand` level and never reach the geometry below.
    //! These pin the exact points the helpers emit so the flat-`f32` argument
    //! lists could be replaced with `Pos2`/`TSTransform` without silently
    //! moving any icon. The expected values were captured from the original
    //! implementation; they are a record of behaviour, not of intent, so a
    //! deliberate change to the curve maths means recapturing them.

    /// Scale and offset chosen to be awkward — non-integer scale, non-zero
    /// origin — so a dropped or swapped transform term cannot pass unnoticed.
    const SCALE: f32 = 2.5;

    fn xf() -> egui::emath::TSTransform {
        egui::emath::TSTransform::new(egui::vec2(10.0, 4.0), SCALE)
    }

    #[track_caller]
    fn assert_points(actual: &[egui::Pos2], expected: &[(f32, f32)]) {
        assert_eq!(actual.len(), expected.len(), "point count");
        for (i, (p, (x, y))) in actual.iter().zip(expected).enumerate() {
            assert_eq!((p.x, p.y), (*x, *y), "point {i}");
        }
    }

    #[test]
    fn map_point_applies_scale_then_offset() {
        assert_points(
            &[super::map_point(xf(), 3.0, -7.0)],
            &[(17.5, -13.5)],
        );
    }

    #[test]
    fn cubic_tessellates_to_eight_segments() {
        let mut points = Vec::new();
        super::tessellate_cubic(
            &mut points,
            egui::pos2(1.0, 2.0),
            egui::pos2(4.0, 8.0),
            egui::pos2(12.0, 3.0),
            egui::pos2(16.0, 9.0),
            xf(),
        );
        assert_points(&points, &[
            (15.854492, 13.443359),
            (20.117188, 15.953125),
            (25.024414, 17.173828),
            (30.3125, 17.75),
            (35.717773, 18.326172),
            (40.976563, 19.546875),
            (45.825195, 22.05664),
            (50.0, 26.5),
        ]);
    }

    #[test]
    fn quad_tessellates_to_six_segments() {
        let mut points = Vec::new();
        super::tessellate_quad(
            &mut points,
            egui::pos2(0.0, 0.0),
            egui::pos2(6.0, 14.0),
            egui::pos2(18.0, 2.0),
            xf(),
        );
        assert_points(&points, &[
            (15.416667, 13.861112),
            (21.666668, 20.11111),
            (28.75, 22.75),
            (36.66667, 21.777777),
            (45.416668, 17.194445),
            (55.0, 9.0),
        ]);
    }

    /// Large-arc, non-sweep, with an x-axis rotation — the branch that
    /// exercises the endpoint-to-center parameterisation and
    /// `arc_segment_to_cubic`.
    #[test]
    fn rotated_large_arc_tessellates() {
        let mut points = Vec::new();
        super::tessellate_arc(
            &mut points,
            egui::pos2(2.0, 3.0),
            egui::pos2(15.0, 11.0),
            super::ArcParams {
                radii: egui::vec2(7.0, 5.0),
                x_rotation_deg: 30.0,
                large_arc: true,
                sweep: false,
            },
            xf(),
        );
        assert_points(&points, &[
            (14.474627, 12.53105), (14.075131, 13.78431), (13.804415, 15.197931),
            (13.665376, 16.710064), (13.660913, 18.25885), (13.79393, 19.782448),
            (14.067323, 21.219), (14.483992, 22.506657), (15.118507, 23.78123),
            (16.008644, 25.180754), (17.102116, 26.646683), (18.346626, 28.120481),
            (19.68989, 29.543612), (21.079615, 30.857538), (22.463509, 32.003723),
            (23.789282, 32.923626), (25.211996, 33.6951), (26.870335, 34.42106),
            (28.687454, 35.080574), (30.586498, 35.65271), (32.49062, 36.11653),
            (34.32297, 36.45111), (36.0067, 36.635506), (37.46496, 36.648792),
            (38.842464, 36.465244), (40.297577, 36.092384), (41.773895, 35.55915),
            (43.215042, 34.894478), (44.564613, 34.127293), (45.766224, 33.28653),
            (46.76348, 32.401123), (47.499996, 31.500008),
        ]);
    }

    /// A zero radius degenerates to a straight line to the endpoint.
    #[test]
    fn zero_radius_arc_becomes_a_line() {
        let mut points = Vec::new();
        super::tessellate_arc(
            &mut points,
            egui::pos2(2.0, 3.0),
            egui::pos2(15.0, 11.0),
            super::ArcParams {
                radii: egui::vec2(0.0, 5.0),
                x_rotation_deg: 0.0,
                large_arc: false,
                sweep: true,
            },
            xf(),
        );
        assert_points(&points, &[(47.5, 31.5)]);
    }
}
