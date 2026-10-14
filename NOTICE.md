# Notice

This crate is a standalone copy of `ext/io` from
[denoland/deno](https://github.com/denoland/deno) at tag `v2.9.7`
(`deno_io` 0.169.0), MIT licensed. See `LICENSE.md`.

Changes from upstream:

- Workspace dependency versions are pinned explicitly, because the crate is
  built outside the Deno monorepo.
- `rand` is pinned to `=0.8.6` (upstream: `=0.8.5`), on Windows only. Cargo
  resolves target-specific dependencies for every platform, so this is needed
  to remove `rand` 0.8.5 from the lockfile. The newer release fixes an
  unsoundness with custom loggers that use `rand::rng()`.
- `deno_subprocess_windows` resolves to the Frontal Labs fork of that crate.
