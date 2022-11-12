#!/bin/sh

# Script to run souffle-lint in CI. Usage:
#
#   curl -sSL https://raw.github.com/langston-barrett/souffle-lint/main/scripts/run.sh | sh -s lint file.dl
#
# Set SOUFFLE_LINT_VERSION in the environment of `sh` to download a specific
# version of souffle-lint (default is the latest).
#
# Arguments after `-s` are passed to souffle-lint.

souffle_lint="$(mktemp -d)/souffle-lint"
cleanup() { rm -rf "${souffle_lint}"; }
always() { trap "${1}" EXIT HUP INT QUIT ABRT ALRM TERM; }
always cleanup

version=${SOUFFLE_LINT_VERSION:-v0.4.0}
bin_url="https://github.com/langston-barrett/souffle-lint/releases/download/${version}/souffle-lint"
echo "${bin_url}"
curl -L -o "${souffle_lint}" "${bin_url}"
chmod +x "${souffle_lint}"
"${souffle_lint}" "${@}"
