# gibcite

Give details of citation from local [Zotero](https://www.zotero.org/) database based on the assigned [Better BibTeX](https://retorque.re/zotero-better-bibtex/) citation key. This command has been tested to work both with Zotero running and not.

The data is returned in a [CSL JSON](https://citationstyles.org/) format, currently includes only:

- Title
- Authors
- Issue date
- Abstract

This program was originally developed for using with [Dendron](https://www.dendron.so), you can see [instructions on how to use it this way](./dendron). But there is nothing dendron-specific to it, might be useful for other integrations.
