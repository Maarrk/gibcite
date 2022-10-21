# gibcite

Give details of citation from local [Zotero](https://www.zotero.org/) database based on the assigned [Better BibTeX](https://retorque.re/zotero-better-bibtex/) citation key. This command has been tested to work both with Zotero running and not.

The data is returned in a [CSL JSON](https://citationstyles.org/) format, currently includes only:

- Title
- Authors
- Issue date
- Abstract

This program was originally developed for using with [Dendron](https://www.dendron.so). Now this is mostly obsolete, as the same thing [can be done with just the hook](https://gist.github.com/Maarrk/01172c1689897979b944c05f8ca948b3) since [this PR](https://github.com/dendronhq/dendron/pull/3001).
