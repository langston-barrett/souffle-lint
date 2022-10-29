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

- Create branch with a name starting with ``release``
- Update ``doc/changelog.rst``
- Update the version number in ``Cargo.toml``, then ``cargo build --release``
- Check that CI was successful on the release branch
- Merge the release branch to ``main``
- Delete the release branch
- ``git checkout main && git pull origin && git tag -a vX.Y.Z -m vX.Y.Z && git push --tags``
- Verify that the release artifacts work as intended
- Release the pre-release created by CI
- Check that the crates were properly uploaded to crates.io
