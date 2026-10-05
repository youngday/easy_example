# Notice: vendored `egui-plotter`

This directory is a **vendored copy** of [`egui-plotter`](https://github.com/Gip-Gip/egui-plotter),
MIT licensed (see `LICENSE`), taken from upstream commit `6740484` on branch `main`
(crate version `0.7.0`).

## What is verbatim

- `README.md` — identical to upstream
- `LICENSE` — identical to upstream

## What was changed

`Cargo.toml`:

- `egui` requirement relaxed from `">=0.32.1, <0.35"` to `"0.36"`.
  Upstream has no egui 0.36 support yet (crates.io stops at `0.6.0`, and upstream
  `main` caps egui below `0.35`), but the library compiles cleanly against
  egui 0.36 apart from unused-import warnings.
- Removed upstream `[dev-dependencies]`, `[[example]]` sections and docs.rs metadata,
  which are not needed for a vendored library dependency.
- `publish = false` so this copy is never pushed to crates.io.

`src/charts/xytime.rs`, `src/charts/timedata.rs` — live/streaming API, so a
chart can be updated in place instead of being rebuilt for every sample:

- `XyTimeData::empty` / `TimeData::empty` — build an empty chart (upstream
  `new` panics on an empty slice).
- `push` — append a point/sample, keeping the data ordered by time and
  refreshing the chart configuration.
- `set_window` / `window` — keep only the N most recent points (a rolling data
  window); plus `len` / `is_empty`.
- Degenerate (single-point / flat) ranges are padded so plotters never receives
  an empty range, and `start_time` / `end_time` return `0.0` on an empty chart
  instead of panicking.

## Why vendored instead of a path dependency

The examples `egui_3d` and `egui_timechart` in this repository need an egui 0.36
plotting backend. Patching the sibling `../egui-plotter` checkout was the
alternative; it was rejected in favour of keeping every change inside this
repository.

## Updating

Re-copy `README.md` and `LICENSE` from upstream, then re-apply the changes above
(the `egui` requirement in `Cargo.toml`, and the live/streaming API in
`src/charts/`).
