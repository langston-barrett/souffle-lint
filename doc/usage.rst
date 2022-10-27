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

On the C Pre-Processor
======================

Soufflé runs the C pre-processor (CPP) on Datalog files by default, and it is
customary for developers to use its functionality. The ``souffle-lint`` parser
does not handle CPP directives, but it's still recommended to run
``souffle-lint`` *before* you run the pre-processor, for the following reasons.

The parser is highly robust to syntax errors and ``souffle-lint`` will ignore
them by default.

Running the C pre-processor before ``souffle-lint`` can result in false
positives, e.g. you might have

.. code-block::

   fact(MACRO_VAR + 1).

where ``MACRO_VAR`` is a definition given to the pre-processor. If you run the
pre-processor before ``souffle-lint`` and define ``MACRO_VAR`` to be an integer
like ``5``, this fact would run afowl of the ``simpl-binop-const`` rule.

Furthermore, ``souffle-lint`` doesn’t (`yet <17_>`_) take advantage of the
``#line`` directives in the CPP output, so the line numbers in its output won’t
correspond to your source file if you run the pre-processor first.

.. _17: https://github.com/langston-barrett/souffle-lint/issues/17
