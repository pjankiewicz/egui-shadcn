//! Show method for AreaChart — renders smooth stacked area curves.

impl super::area_chart::AreaChart {
    /// Renders the area chart. Returns the response for the allocated rect.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());

        let available_width = ui.available_width();
        let (chart_rect, response) = ui.allocate_exact_size(
            egui::vec2(available_width, self.height),
            egui::Sense::hover(),
        );

        if !ui.is_rect_visible(chart_rect) || self.series.is_empty() {
            return response;
        }

        let painter = ui.painter_at(chart_rect);
        let bottom_pad = 24.0;
        let plot_rect = egui::Rect::from_min_max(
            chart_rect.min,
            egui::pos2(chart_rect.max.x, chart_rect.max.y - bottom_pad),
        );

        let n = self.labels.len();
        if n == 0 {
            return response;
        }

        let step_x = plot_rect.width() / (n as f32 - 1.0).max(1.0);

        // Compute Y-axis max
        let y_max = self.compute_y_max(n);

        // Horizontal grid lines
        for i in 0..5 {
            let t = i as f32 / 4.0;
            let y = plot_rect.max.y - t * plot_rect.height();
            painter.hline(
                plot_rect.x_range(),
                y,
                egui::Stroke::new(0.5, theme.border),
            );
        }

        // Compute screen-space Y values per series
        let raw_curves = self.compute_raw_curves(n, step_x, plot_rect, y_max);

        // Smooth each curve and draw fills + lines
        let mut prev_smooth: Option<Vec<egui::Pos2>> = None;

        for (series_idx, raw_points) in raw_curves.iter().enumerate() {
            let smooth = catmull_rom_to_smooth(raw_points);
            let color = self.series[series_idx].color;

            let fill_alpha = if series_idx == 0 { 40 } else { 25 };
            let fill_color = egui::Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                fill_alpha,
            );

            let line_alpha = if series_idx == 0 { 180 } else { 120 };
            let line_color = egui::Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                line_alpha,
            );

            // Build baseline: previous series smooth curve, or flat bottom
            let baseline: Vec<egui::Pos2> = match &prev_smooth {
                Some(prev) => prev.clone(),
                None => smooth
                    .iter()
                    .map(|p| egui::pos2(p.x, plot_rect.max.y))
                    .collect(),
            };

            // Fill using a triangle-strip mesh (handles concave shapes correctly)
            fill_between_curves(&painter, &smooth, &baseline, fill_color);

            // Line on top
            if smooth.len() >= 2 {
                painter.add(egui::Shape::line(
                    smooth.clone(),
                    egui::Stroke::new(1.5, line_color),
                ));
            }

            prev_smooth = Some(smooth);
        }

        // X-axis labels
        let label_count = 8.min(n);
        let label_step = (n - 1).max(1) / label_count.max(1);
        for i in (0..n).step_by(label_step.max(1)) {
            let x = plot_rect.min.x + i as f32 * step_x;
            painter.text(
                egui::pos2(x, chart_rect.max.y - 4.0),
                egui::Align2::CENTER_BOTTOM,
                &self.labels[i],
                egui::FontId::proportional(10.0),
                theme.muted_foreground,
            );
        }

        response
    }

    fn compute_y_max(&self, n: usize) -> f32 {
        if self.stacked {
            (0..n)
                .map(|i| {
                    self.series
                        .iter()
                        .map(|s| s.values.get(i).copied().unwrap_or(0.0))
                        .sum::<f32>()
                })
                .fold(0.0_f32, f32::max)
                .max(1.0)
        } else {
            self.series
                .iter()
                .flat_map(|s| s.values.iter().copied())
                .fold(0.0_f32, f32::max)
                .max(1.0)
        }
    }

    fn compute_raw_curves(
        &self,
        n: usize,
        step_x: f32,
        plot_rect: egui::Rect,
        y_max: f32,
    ) -> Vec<Vec<egui::Pos2>> {
        let mut cumulative = vec![0.0_f32; n];
        let mut curves = Vec::with_capacity(self.series.len());

        for series in &self.series {
            let points: Vec<egui::Pos2> = (0..n)
                .map(|i| {
                    let val = series.values.get(i).copied().unwrap_or(0.0);
                    let stacked_val = if self.stacked {
                        cumulative[i] + val
                    } else {
                        val
                    };
                    let x = plot_rect.min.x + i as f32 * step_x;
                    let y = plot_rect.max.y - (stacked_val / y_max) * plot_rect.height();
                    egui::pos2(x, y)
                })
                .collect();

            if self.stacked {
                for (i, total) in cumulative.iter_mut().enumerate().take(n) {
                    *total += series.values.get(i).copied().unwrap_or(0.0);
                }
            }

            curves.push(points);
        }

        curves
    }
}

/// Fills the area between two curves using a triangle-strip mesh.
/// Both curves must have the same length and matching X coordinates.
fn fill_between_curves(
    painter: &egui::Painter,
    top: &[egui::Pos2],
    bottom: &[egui::Pos2],
    color: egui::Color32,
) {
    if top.len() < 2 || top.len() != bottom.len() {
        return;
    }

    let mut mesh = egui::Mesh::default();
    let white_uv = egui::epaint::WHITE_UV;

    // Add all vertices: top curve then bottom curve
    for &p in top {
        mesh.vertices.push(egui::epaint::Vertex {
            pos: p,
            uv: white_uv,
            color,
        });
    }
    for &p in bottom {
        mesh.vertices.push(egui::epaint::Vertex {
            pos: p,
            uv: white_uv,
            color,
        });
    }

    let n = top.len() as u32;
    // Build triangle strip: for each column i, create two triangles
    // forming a quad between top[i], top[i+1], bottom[i], bottom[i+1]
    for i in 0..(n - 1) {
        let t0 = i;           // top[i]
        let t1 = i + 1;       // top[i+1]
        let b0 = n + i;       // bottom[i]
        let b1 = n + i + 1;   // bottom[i+1]

        mesh.indices.extend_from_slice(&[t0, b0, t1]);
        mesh.indices.extend_from_slice(&[t1, b0, b1]);
    }

    painter.add(egui::Shape::mesh(mesh));
}

/// Converts raw data points into a smooth curve using Catmull-Rom → cubic Bezier conversion.
/// Each cubic segment is tessellated into 8 line segments.
fn catmull_rom_to_smooth(points: &[egui::Pos2]) -> Vec<egui::Pos2> {
    let n = points.len();
    if n <= 2 {
        return points.to_vec();
    }

    let segments_per_curve = 8;
    let mut smooth = Vec::with_capacity((n - 1) * segments_per_curve + 1);
    smooth.push(points[0]);

    for i in 0..(n - 1) {
        let p_prev = if i == 0 { points[0] } else { points[i - 1] };
        let p_curr = points[i];
        let p_next = points[i + 1];
        let p_next2 = if i + 2 < n { points[i + 2] } else { points[n - 1] };

        // Catmull-Rom → cubic Bezier control points
        let cp1 = egui::pos2(
            p_curr.x + (p_next.x - p_prev.x) / 6.0,
            p_curr.y + (p_next.y - p_prev.y) / 6.0,
        );
        let cp2 = egui::pos2(
            p_next.x - (p_next2.x - p_curr.x) / 6.0,
            p_next.y - (p_next2.y - p_curr.y) / 6.0,
        );

        // Tessellate the cubic Bezier
        for seg in 1..=segments_per_curve {
            let t = seg as f32 / segments_per_curve as f32;
            let pt = cubic_bezier(p_curr, cp1, cp2, p_next, t);
            smooth.push(pt);
        }
    }

    smooth
}

/// Evaluates a cubic Bezier curve at parameter t.
fn cubic_bezier(
    p0: egui::Pos2,
    p1: egui::Pos2,
    p2: egui::Pos2,
    p3: egui::Pos2,
    t: f32,
) -> egui::Pos2 {
    let u = 1.0 - t;
    let uu = u * u;
    let uuu = uu * u;
    let tt = t * t;
    let ttt = tt * t;

    egui::pos2(
        uuu * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + ttt * p3.x,
        uuu * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + ttt * p3.y,
    )
}
