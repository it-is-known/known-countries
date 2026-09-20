# Project guide

Work within this repository; do not inspect parent directories.
This is a generated `Country` library for Rust, Dart, Python, and Ruby.
`js/` is scaffolding only.

## Source of truth

- `data/countries.csv`: headerless `alpha2,alpha3,name`, lowercase codes,
  sorted by alpha2. It is a selected subset of ISO 3166-1, not a complete registry.
- `.config/codegen/<language>/country.liquid`: implementation and API docs.
  Edit templates first; never change generated implementations alone.

| Language | Generated output |
| --- | --- |
| Rust | `rust/src/country.rs` |
| Dart | `dart/lib/src/country.dart` |
| Python | `python/src/known_countries/__init__.py` |
| Ruby | `ruby/lib/known/countries.rb` |

From the repository root, run `rake -B <output-path>` using a path above.
Requires Ruby 3.4+ and the `rake`, `csv`, and `lvr` gems.
Regenerate all four outputs after CSV or shared-generator changes.
Review both template and generated diffs.

`Rakefile` defines generation. Bare `rake` also renders READMEs with `readmer`;
root-context rendering currently fails. Root `make` only renders the README.
Use targeted output tasks for code changes.

## Compatibility and documentation

- CSV names generate public identifiers. Row order affects Rust ordering and
  Borsh variant tags; assess compatibility when adding, renaming, or sorting rows.
- Rust: edition 2024, MSRV 1.85, `#![no_std]`, no unsafe code. Gate heap use on
  `alloc` and standard-library use on `std`. Keep default builds dependency-free.
  The `all` feature is empty; it is not equivalent to `--all-features`.
- Preserve lowercase, case-sensitive code lookup unless changing the API is
  requested. Rust `FromStr` rejects unknown codes; `From<&str>` preserves them
  as `Other` with `alloc`, but panics without it. Rust defaults to United States.
- Document public symbols beside the code: Rust `rustdoc`, Dart doc comments,
  Python docstrings, Ruby YARD. Include feature gates, errors/panics, and useful
  examples. Generated API documentation belongs in the templates.
- Keep READMEs terse; additions need clear value. Essential README changes go
  in `.config/readmer/` templates and their rendered outputs.

## Checks

Run relevant checks inside the indicated package directory:

- `rust/`: `cargo test --locked`, `cargo check --locked --no-default-features`,
  `cargo check --locked --all-features`, `cargo fmt --check`.
  For feature changes, also check individual integrations without defaults,
  especially `cargo check --locked --no-default-features --features alloc,serde`.
- `dart/` (Dart 3.11+): `dart pub get`, `dart analyze`, `dart test`,
  `dart format --output=none --set-exit-if-changed lib test example`.
- `python/` (Python 3.11+): `uv lock --check`; exercise `Country` from `src/`.
  There is no test suite yet.
- `ruby/` (Ruby 3.4+): `bundle install`, then `bundle exec rspec`.

Existing suites are mostly empty; successful compilation is not behavior
coverage. Report zero-test runs and baseline failures explicitly. CI currently
checks only Rust. CSV/shared-template changes require checking all four bindings.
