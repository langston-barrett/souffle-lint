=====
Usage
=====

Pass a list of Soufflé Datalog files to ``souffle-lint lint``, or pass
one on stdin:

.. code-block:: bash

   # Lint a single file:
   souffle-lint lint file.dl
   # Lint a file from stdin:
   souffle-lint lint <file.dl
   # Lint two files:
   souffle-lint lint file.dl path/to/file2.dl
   # Lint all the Soufflé Datalog files the current directory:
   souffle-lint lint ./**/*.dl

The exit code will be ``0`` if ``souffle-lint`` succeeded with no warnings,
``1`` if there were any warnings, or ``2`` if there was a problem (e.g., a parse
error or bad configuration).

See ``--help`` for more options and :doc:`config` for configuration.

Pre-Processing
==============

Soufflé runs the C pre-processor (CPP) on Datalog files by default, and it is
customary for developers to use its functionality. The ``souffle-lint`` parser
does not handle CPP directives; if you use them you must run the pre-processor
manually and then run ``souffle-lint`` on the result. For instance,

.. code-block:: bash

   mcpp -W 0 -P your/datalog/file.dl -o out.dl
   souffle-lint lint out.dl

``souffle-lint`` doesn’t (`yet <17_>`_) take advantage of the ``#line``
directives in the CPP output, so the line numbers in its output won’t correspond
to your source file.

.. _17: https://github.com/langston-barrett/souffle-lint/issues/17
