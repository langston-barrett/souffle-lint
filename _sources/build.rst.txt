=====
Build
=====

To build souffle-lint from source, get the source:

.. code-block:: bash

   git clone https://github.com/langston-barrett/souffle-lint
   cd souffle-lint

Then install build-time dependencies:

- Cargo and rustc (see `rustup <rustup_>`_)

- Sphinx: Build manual (embedded into binary)

  * Make: Needed for Sphinx build

.. code-block:: bash

   virtualenv venv
   source venv/bin/active
   pip install -r doc/requirements.txt

Finally, build the code:

.. code-block:: bash

   cargo build --release

To install, just copy the binary somewhere:

.. code-block:: bash

   cp target/release/souffle-lint /usr/bin

.. _rustup: https://rustup.rs/
