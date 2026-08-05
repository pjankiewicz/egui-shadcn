# Vendoring notes

This is a vendored fork of [egui-shadcn](https://github.com/pjankiewicz/egui-shadcn) by
Pawel Jankiewicz (MIT, same licence as this workspace).

- **Upstream**: https://github.com/pjankiewicz/egui-shadcn
- **Forked at**: `fa5ceee` — *Add public paint_icon_svg for custom Lucide-grammar icons*
- **Why vendored**: upstream targets egui 0.33; this workspace is on egui 0.35. A path
  dependency also needs to be tracked in git — Cargo fails to resolve the manifest on a
  clean clone if the directory is missing, even when the `shadcn-pilot` feature that
  consumes it is off.

## Local delta against upstream

Everything below is API drift, not behaviour change. Upstream's clippy warnings are left
untouched on purpose so the fork stays easy to rebase.

### `Cargo.toml`

| Dependency | Upstream | Here |
|---|---|---|
| `egui` | 0.33 | 0.35 |
| `egui_flex` | 0.5 | 0.7 |
| `eframe` (dev) | 0.33 | 0.35, `default-features = false`, `features = ["default_fonts", "wgpu"]` |

The `eframe` feature set must match `src_rust/Cargo.toml`'s. Cargo unifies dev-dependency
features across the workspace, so a plain `eframe = "0.35"` would pull glow and the full
default backend into the editor build.

### Library

- `src/theme/shadcn_theme_ext.rs` — `Context::style_mut` was removed in egui 0.35, which
  moved to per-theme styles. Replaced with `all_styles_mut`, which keeps the old
  single-style semantics.
- `src/widgets/input/input_widget_impl.rs`,
  `src/widgets/textarea/textarea_widget_impl.rs`,
  `src/widgets/input_group/input_group_show.rs` — `TextEdit::frame` changed from `bool` to
  `Frame` in egui 0.34.

  `.frame(false)` is **not** `.frame(Frame::NONE)`. The old boolean only suppressed
  painting; the widget's default `margin: Margin::symmetric(4, 2)` still applied. In 0.34+ a
  custom `Frame` replaces `margin` outright, so a bare `Frame::NONE` shifts text 4px left and
  2px up. These sites use `Frame::NONE.inner_margin(Margin::symmetric(4, 2))` to stay
  pixel-identical to upstream.

### Examples

`examples/{demo,shadcn_demo,dashboard,component_dashboard}.rs` — eframe/egui 0.35 drift,
the same migration `crates/rpg_editor_app/src/main.rs` and `crates/rpg_editor_ui/src/root.rs`
already went through:

- `eframe::App::update(&mut self, ctx: &Context, _)` → `App::ui(&mut self, ui: &mut Ui, _)`
- `egui::SidePanel::left` / `egui::TopBottomPanel::top` → `egui::Panel::left` / `Panel::top`
- `Panel::show(ctx, …)` → `Panel::show(ui, …)`
- `Panel::exact_width` / `default_width` → `exact_size` / `default_size` (the unified
  `Panel` measures along its own axis, so the names dropped the direction)

`component_dashboard` keeps a `let ctx = ui.ctx().clone()` because this crate's overlay
widgets — `Dialog`, `AlertDialog`, `Sheet`, `Drawer`, `Command`, `Toast` — are still
`Context`-driven and unaffected by the panel change.

## Behaviour worth knowing before using more widgets

- `ShadcnThemeExt::set_shadcn_theme` calls `all_styles_mut` and overwrites `window_fill`,
  `window_stroke`, and `window_shadow`. `rpg_editor_ui::theme::apply` calls it *before*
  `ctx.set_visuals`, so the editor's visuals win.
- `setup_fonts` installs Geist as the primary proportional font for the entire app. The
  editor does not call it.
- `Spinner`, `Skeleton`, and `Toast` call `request_repaint()` unconditionally every frame.
  Using any of them keeps the editor awake; `rpg_editor_ui` has an idle-repaint regression
  test that guards against this.
