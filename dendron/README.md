# Instructions for usage with Dendron

## Prerequisites

Ensure you have these:

- Visual Studio Code
  - Dendron
  - Citation Picker for Zotero
- Zotero
  - Better BibTeX
- Download appropriate `gibcite` binary from the [Releases page](https://github.com/Maarrk/gibcite/releases)
  - _Optional: put it into PATH_

## Setup

- Change Citation Picker "Port" setting:

```json
"zotero-citation-picker.port": "http://127.0.0.1:23119/better-bibtex/cayw?format=playground&keyprefix=[[bib.&keypostfix=]]"
```

- [Create a hook in Dendron](https://wiki.dendron.so/notes/070f5adf-3ea3-4e83-b468-75d1b4b6094a/) with these settings in `dendron.yml`:

<!-- prettier-ignore -->
```yaml
    hooks:
        onCreate:
            -
                id: newbib
                pattern: bib.*
                type: js
```

- Replace the `newbib.js` hook with [the one from this repo](./newbib.js)
- If the command `gibcite` is not in your PATH, include the whole location of the executable into the hook, like this:

```js
await execa("/full/path/to/gibcite", [note.title]);
```

## Usage

- Add a citation with Zotero Citation Picker
- Once you get the `[[bib.smithExamplePaper2022]]` links, visit them to create filled notes
