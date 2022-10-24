=============
Writing Rules
=============

The `YAML <https://yaml.org/>`__ configuration file consists of a list of
*rules*. Each rule has:

-  A short name
-  A short description
-  A list of *queries*
-  A list of examples

Here is an example of a simple rule which exhorts its user to simplify
compile-time constant additions:

.. code-block:: yaml

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
         left: (constant)
         operator: "+"
         right: (constant))

You can view the built-in configuration file at ```./config/default.yml``.

A query describes a pattern in the program’s concrete syntax tree (CST), a rule
triggers a warning when the query matches its CST. Queries are written in the
`tree-sitter query language
<https://tree-sitter.github.io/tree-sitter/using-parsers#query-syntax>`__.

The name should be less than 30 characters, the short description should be less
than 60 characters.

Showing the Parse Tree
======================

You can see the tree-sitter S-expression (i.e., concrete syntax tree)
corresponding to Datalog source file(s) using the ``sexp`` command:

.. code-block:: bash

   souffle-lint sexp file.dl

See ``souffle-lint sexp --help`` for more details and other options.
