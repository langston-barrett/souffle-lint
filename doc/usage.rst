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

See ``--help`` for more options.

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

Ignoring Rules
==============

Each rule has an associated name. These names are visible in the list shown by
the ``info`` command, and also in warning messages. For example, in the
following message, the rule name is ``error-dup-type-decl``:

::

   warn[error-dup-type-decl] Duplicated type declaration

You can disable a rule entirely by passing ``--ignore=<rule-name>`` on the
command line. In fact, you can pass a *prefix* to ``--ignore``; any rules that
start with that prefix will be ignored. For example, ``--ignore=simpl`` will
disable any rules with names starting with ``simpl``. See :doc:`categories` for
information about common prefixes.

Ignoring a specific warning on a specific line is `not yet implemented
<https://github.com/langston-barrett/souffle-lint/issues/5>`_.

..
   You can ignore a warning for a specific line by by placing a comment of the
   form `ignore[<warning-name>]` on the line before, e.g.,

   .. code-block:: bash

      // ignore[simpl-binop-id]
      one(0 + 1).

Rules with slow execution times are disabled by default, you can enable them
with ``souffle-lint lint --slow``.
