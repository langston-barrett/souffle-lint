============
Installation
============

Once you've installed ``souffle-lint``, head to :doc:`usage`.

Install from a Release
======================

Statically-linked binaries and Debian packages are available on the
`releases page`_.

Install with Cargo
==================

You can also install souffle-lint via Cargo and crates.io:

.. code-block:: bash

  cargo install souffle-lint

Install in CI
=============

souffle-lint provides a script for easy use in CI. Just add this snippet to
your CI job:

.. code-block:: bash

  curl -sSL https://raw.github.com/langston-barrett/souffle-lint/main/scripts/ci.sh | \
    sh -s lint file.dl

Set ``SOUFFLE_LINT_VERSION`` in the environment of ``sh`` to download a
specific version of souffle-lint (the default is the latest).

Arguments after ``-s`` are passed to souffle-lint.

Build and Install from Source
=============================

See :doc:`build`.

.. _releases page: https://github.com/langston-barrett/souffle-lint/releases
