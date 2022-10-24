=============
Configuration
=============

Adding Rules
============

You can add new rules to a YAML configuration file and pass it to ``lint`` via
the ``--config`` flag:

.. code-block:: bash

  souffle-lint lint --config rules.yml program.dl

See :doc:`writing` for how to write rules.

Ignoring Rules
==============

Ignoring a specific warning on a specific line is `not yet implemented
<https://github.com/langston-barrett/souffle-lint/issues/5>`_.

Via the CLI
-----------

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

Via a Configuration File
------------------------

You can also add a key ``ignore:`` to a YAML configuration file:

.. code-block:: yaml

  ignore:
  - some-rule-name
  - another-rule-name

and pass ``--config`` to ``lint``

.. code-block:: bash

  souffle-lint lint --config ignores.yml program.dl

..
   You can ignore a warning for a specific line by by placing a comment of the
   form `ignore[<warning-name>]` on the line before, e.g.,

   .. code-block:: bash

      // ignore[simpl-binop-id]
      one(0 + 1).

Slow Rules
==========

Rules with slow execution times are disabled by default, you can enable them
with ``souffle-lint lint --slow``.
