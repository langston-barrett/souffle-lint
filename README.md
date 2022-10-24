# souffle-lint

A linter for [Soufflé Datalog][souffle], based on
[tree-sitter-souffle][tree-sitter-souffle], configured with
[tree-sitter queries][tree-sitter-query].

## Contents

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [souffle-lint](#souffle-lint)
    - [Contents](#contents)
    - [Build and Install](#build-and-install)
        - [Install from a Release](#install-from-a-release)
        - [Build From Source](#build-from-source)
    - [Usage](#usage)
        - [Pre-Processing](#pre-processing)
    - [Ignoring Rules](#ignoring-rules)
    - [Writing Rules](#writing-rules)
        - [Showing the Parse Tree](#showing-the-parse-tree)
    - [Rule Categories](#rule-categories)
    - [Development](#development)
        - [Tests](#tests)
        - [Benchmarks](#benchmarks)
        - [Releasing](#releasing)

<!-- markdown-toc end -->

## Build and Install

<!-- TODO(lb): cargo install -->
<!-- TODO(lb): deb install -->
<!-- TODO(lb): nix install -->

### Install from a Release

Statically-linked binaries are available on the [releases page][releases].

### Build From Source

```bash
git clone https://github.com/langston-barrett/souffle-lint
cd souffle-lint
cargo build --release
```
To install, just copy the binary somewhere:
```
cp target/release/souffle-lint /usr/bin
```

## Usage

Pass a list of Soufflé Datalog files to `souffle-lint lint`, or pass one on
stdin:

```bash
# Lint a single file:
souffle-lint lint file.dl
# Lint a file from stdin:
souffle-lint lint <file.dl
# Lint two files:
souffle-lint lint file.dl path/to/file2.dl
# Lint all the Soufflé Datalog files the current directory:
souffle-lint lint ./**/*.dl
```

The exit code will be `0` if `souffle-lint` succeeded with no warnings, `1` if
there were any warnings, or `2` if there was a problem (e.g., a parse error or
bad configuration).

See `--help` for more options.

### Pre-Processing

Soufflé runs the C pre-processor (CPP) on Datalog files by default, and it is
customary for developers to use its functionality. The `souffle-lint` parser
does not handle CPP directives; if you use them you must run the
pre-processor manually and then run `souffle-lint` on the result. For instance,
```bash
mcpp -W 0 -P your/datalog/file.dl -o out.dl
souffle-lint lint out.dl
```
`souffle-lint` doesn't (yet) take advantage of the `#line` directives in the CPP
output, so the line numbers in its output won't correspond to your source file.

<!-- TODO(lb): Use `#line` -->

## Ignoring Rules

Each rule has an associated name. These names are visible in the list shown by
the `info` command, and also in warning messages. For example, in the following
message, the rule name is `error-dup-type-decl`:

```
warn[error-dup-type-decl] Duplicated type declaration
```

You can disable a rule entirely by passing `--ignore=<rule-name>` on the command
line. In fact, you can pass a *prefix* to `--ignore`; any rules that start with
that prefix will be ignored. For example, `--ignore=simpl` will disable any
rules with names starting with `simpl`. See [Rule Categories](#rule-categories)
for information about common prefixes.

Ignoring a specific warning on a specific line is [not yet implemented][#5].

<!-- TODO(#5): Implement me! 

You can ignore a warning for a specific line by by placing a comment of the form `ignore[<warning-name>]` on the line before, e.g.,
```
// ignore[simpl-binop-id]
one(0 + 1).
```

-->

Rules with slow execution times are disabled by default, you can enable them
with `souffle-lint lint --slow`.

## Writing Rules

The [YAML][yaml] configuration file consists of a list of *rules*. Each rule
has:

- A short name
- A short description
- A list of *queries*
- A list of examples

Here is an example of a simple rule which exhorts its user to simplify
compile-time constant additions:

```yaml
rules:
- name: simpl-binop-plus
  short: Simplify constant binary operation (+)
  long: >
    Some extended description of this rule.
  examples:
    - before: |
        even(2 + 2).
      after: |
        even(4).
  queries:
  - |
    (binary_op
      left: (argument (constant))
      operator: "+"
      right: (argument (constant)))
```

You can view the built-in configuration file at
[`./config/default.yml`](./config/default.yml).

A query describes a pattern in the program's concrete syntax tree (CST), a rule
triggers a warning when the query matches its CST. Queries are written in the
[tree-sitter query language][tree-sitter-query].

The name should be less than 30 characters, the short description should be less
than 60 characters.

### Showing the Parse Tree

You can see the tree-sitter S-expression (i.e., concrete syntax tree)
corresponding to Datalog source file(s) using the `sexp` command:
```
souffle-lint sexp file.dl
```
See `souffle-lint sexp --help` for more details and other options.

## Rule Categories

By convention, rules have names starting with common prefixes:

- `depr`: This functionality is deprecated
- `error`: Soufflé would reject this code
  - `error-dup`: Some entity is duplicated
- `simpl`: The code can be simplified, possibly for better performance
  - `simpl-dup`: Some entity is duplicated
- `style`: This is a stylistic choice
  - `style-name`: This is a stylistic choice about names

## Development

### Tests

Tested with [lit][lit] and [FileCheck][filecheck].
```bash
cargo build
lit --path=$PWD/target/debug test/
```

### Benchmarks

Large Soufflé files are available in `bench/`. Try passing `--trace` to
`souffle-lint`. Compare performance to `souffle --show=parse-errors`.

### Releasing

1. Update [`CHANGELOG.md`](./CHANGELOG.md)
2. Update the version number in [`Cargo.toml`](./Cargo.toml)
3. `git checkout main && git pull origin && git tag -a vX.Y.Z -m vX.Y.Z && git push --tags`
4. `cargo publish`
5. Release the pre-release created by CI

[#5]: https://github.com/langston-barrett/souffle-lint/issues/5
[filecheck]: https://www.llvm.org/docs/CommandGuide/FileCheck.html
[lit]: https://llvm.org/docs/CommandGuide/lit.html
[releases]: https://github.com/langston-barrett/souffle-lint/releases
[souffle]: https://souffle-lang.github.io/index.html
[tree-sitter-query]: https://tree-sitter.github.io/tree-sitter/using-parsers#query-syntax
[tree-sitter-souffle]: https://github.com/langston-barrett/tree-sitter-souffle/
[yaml]: https://yaml.org/
