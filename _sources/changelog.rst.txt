=========
Changelog
=========

.. See: https://keepachangelog.com/en/1.0.0/

.. TODO(lb): How to put the hyperlink targets at the end?

`0.2.0 <https://github.com/langston-barrett/souffle-lint/releases/tag/v0.2.0>`_ - 2022-10-24
============================================================================================

`Compare to v0.1.2 <https://github.com/langston-barrett/souffle-lint/compare/v0.1.2...v0.2.0>`_

Added
-----

- Online documentation! https://langston-barrett.github.io/souffle-lint/
- ``man`` subcommand to display documentation via ``man``
- Configuration files may now contain an ``ignore:`` key that works like the
  ``--ignore`` flag of ``lint``.
- Small documentation improvements.

Changed
-------

- ``lint`` now emits a warning on parse errors
- Upgraded to ``souffle-tree-sitter`` v0.4.0, with substantial changes to the
  grammar (i.e., query patterns).


`0.1.2 <https://github.com/langston-barrett/souffle-lint/releases/tag/v0.1.2>`_ - 2022-10-24
============================================================================================

`Compare to v0.1.1 <https://github.com/langston-barrett/souffle-lint/compare/v0.1.1...v0.1.2>`_

Fixed
-----

- Fixed CI uploads of releases

`0.1.1 <https://github.com/langston-barrett/souffle-lint/releases/tag/v0.1.1>`_ - 2022-10-24
============================================================================================

`Compare to v0.1.0 <https://github.com/langston-barrett/souffle-lint/compare/v0.1.0...v0.1.1>`_

Added
-----

- Releases now include Debian packages with manpages

Changed
-------

- ``--interactive`` now properly controls coloration in the ``info`` subcommand
- Minor updates to the README
- Minor performance improvements

`0.1.0 <https://github.com/langston-barrett/souffle-lint/releases/tag/v0.1.0>`_ - 2022-10-21
============================================================================================

First release!
