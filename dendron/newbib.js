// Parts of this file republished from https://github.com/dendronhq/dendron
// Dendron Copyright 2020 Thence LLC. All Rights Reserved.

/**
 @params wsRoot: string, root of your current workspace
 @params note: Object with following properties https://github.com/dendronhq/dendron/blob/master/packages/common-all/src/types/foundation.ts#L66:L66
 @params NoteUtils: utilities for working with notes. [code](https://github.com/dendronhq/dendron/blob/master/packages/common-all/src/dnode.ts#L323:L323)
 @params execa: instance of [execa](https://github.com/sindresorhus/execa#execacommandcommand-options)
 @params _: instance of [lodash](https://lodash.com/docs)
 */
module.exports = async function ({ wsRoot, note, NoteUtils, execa, _ }) {
  const { stdout } = await execa("gibcite", [note.title]);
  const csl = JSON.parse(stdout)[0];

  note.title = csl.title;

  var shortAuthor = "";
  if (csl.author.length === 1) {
    shortAuthor = csl.author[0].family;
  } else if (csl.author.length === 2) {
    shortAuthor = csl.author[0].family + " and " + csl.author[1].family;
  } else {
    shortAuthor = csl.author[0].family + " et al.";
  }
  note.desc = shortAuthor + ", " + csl.issued["date-parts"][0][0].toString();

  note.body =
    note.body + `\n\n## Abstract\n\n${csl.abstract}\n\n## Authors\n\n`;
  for (author of csl.author) {
    note.body = note.body + `- ${author.family}, ${author.given}\n`;
  }

  return { note };
};
