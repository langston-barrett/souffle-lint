===========
Development
===========

See also :doc:`build`.

Tests
=====

Tested with `lit <https://llvm.org/docs/CommandGuide/lit.html>`__ and
`FileCheck <https://www.llvm.org/docs/CommandGuide/FileCheck.html>`__.

.. code-block:: bash

   cargo build
   lit --path=$PWD/target/debug test/

Alternatively,

.. code-block:: bash

   make lit

Benchmarks
==========

Large Soufflé files are available in ``bench/``. Try passing ``--trace`` to
``souffle-lint``. Compare performance to ``souffle --show=parse-errors``.

Releasing
=========

1. Update ```CHANGELOG.md``
2. Update the version number in ```Cargo.toml``
3. ``git checkout main && git pull origin && git tag -a vX.Y.Z -m vX.Y.Z && git push --tags``
4. ``cargo publish``
5. Release the pre-release created by CI
