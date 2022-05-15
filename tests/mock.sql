-- Parts of this file adapted from:
-- https://github.com/zotero/zotero/blob/master/resource/schema/system-107.sql
-- https://github.com/zotero/zotero/blob/master/resource/schema/userdata.sql
--
-- Copyright (c) 2022 Marek S. Łukasiewicz
-- Copyright (c) 2009 Center for History and New Media
--                    George Mason University, Fairfax, Virginia, USA
--                    http://zotero.org
--
--
-- Zotero is free software: you can redistribute it and/or modify
-- it under the terms of the GNU Affero General Public License as published by
-- the Free Software Foundation, either version 3 of the License, or
-- (at your option) any later version.
--
-- Zotero is distributed in the hope that it will be useful,
-- but WITHOUT ANY WARRANTY; without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
-- GNU Affero General Public License for more details.
--
-- You should have received a copy of the GNU Affero General Public License
-- along with Zotero.  If not, see <http://www.gnu.org/licenses/>.

--
-- Tables from the original Zotero file
--

-- Populated at startup from fields and customFields
CREATE TABLE fieldsCombined (
    fieldID INT NOT NULL,
    fieldName TEXT NOT NULL,
    label TEXT,
    fieldFormatID INT,
    custom INT NOT NULL,
    PRIMARY KEY (fieldID)
);
INSERT INTO fieldsCombined VALUES(1,'title',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(2,'abstractNote',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(3,'artworkMedium',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(4,'medium',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(5,'artworkSize',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(6,'date',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(7,'language',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(8,'shortTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(9,'archive',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(10,'archiveLocation',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(11,'libraryCatalog',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(12,'callNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(13,'url',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(14,'accessDate',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(15,'rights',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(16,'extra',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(17,'audioRecordingFormat',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(18,'seriesTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(19,'volume',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(20,'numberOfVolumes',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(21,'place',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(22,'label',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(23,'publisher',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(24,'runningTime',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(25,'ISBN',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(26,'billNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(27,'number',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(28,'code',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(29,'codeVolume',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(30,'section',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(31,'codePages',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(32,'pages',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(33,'legislativeBody',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(34,'session',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(35,'history',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(36,'blogTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(37,'publicationTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(38,'websiteType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(39,'type',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(40,'series',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(41,'seriesNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(42,'edition',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(43,'numPages',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(44,'bookTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(45,'caseName',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(46,'court',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(47,'dateDecided',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(48,'docketNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(49,'reporter',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(50,'reporterVolume',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(51,'firstPage',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(52,'versionNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(53,'system',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(54,'company',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(55,'programmingLanguage',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(56,'proceedingsTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(57,'conferenceName',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(58,'DOI',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(59,'dictionaryTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(60,'subject',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(61,'encyclopediaTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(62,'distributor',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(63,'genre',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(64,'videoRecordingFormat',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(65,'forumTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(66,'postType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(67,'committee',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(68,'documentNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(69,'interviewMedium',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(70,'issue',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(71,'seriesText',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(72,'journalAbbreviation',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(73,'ISSN',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(74,'letterType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(75,'manuscriptType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(76,'mapType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(77,'scale',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(78,'country',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(79,'assignee',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(80,'issuingAuthority',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(81,'patentNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(82,'filingDate',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(83,'applicationNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(84,'priorityNumbers',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(85,'issueDate',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(86,'references',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(87,'legalStatus',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(88,'episodeNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(89,'audioFileType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(90,'repository',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(91,'archiveID',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(92,'citationKey',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(93,'presentationType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(94,'meetingName',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(95,'programTitle',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(96,'network',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(97,'reportNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(98,'reportType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(99,'institution',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(100,'nameOfAct',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(101,'codeNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(102,'publicLawNumber',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(103,'dateEnacted',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(104,'thesisType',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(105,'university',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(106,'studio',NULL,NULL,0);
INSERT INTO fieldsCombined VALUES(107,'websiteTitle',NULL,NULL,0);


-- Defines the possible creator types (contributor, editor, author)
CREATE TABLE creatorTypes (
    creatorTypeID INTEGER PRIMARY KEY,
    creatorType TEXT
);
INSERT INTO creatorTypes VALUES(1,'artist');
INSERT INTO creatorTypes VALUES(2,'contributor');
INSERT INTO creatorTypes VALUES(3,'performer');
INSERT INTO creatorTypes VALUES(4,'composer');
INSERT INTO creatorTypes VALUES(5,'wordsBy');
INSERT INTO creatorTypes VALUES(6,'sponsor');
INSERT INTO creatorTypes VALUES(7,'cosponsor');
INSERT INTO creatorTypes VALUES(8,'author');
INSERT INTO creatorTypes VALUES(9,'commenter');
INSERT INTO creatorTypes VALUES(10,'editor');
INSERT INTO creatorTypes VALUES(11,'translator');
INSERT INTO creatorTypes VALUES(12,'seriesEditor');
INSERT INTO creatorTypes VALUES(13,'bookAuthor');
INSERT INTO creatorTypes VALUES(14,'counsel');
INSERT INTO creatorTypes VALUES(15,'programmer');
INSERT INTO creatorTypes VALUES(16,'reviewedAuthor');
INSERT INTO creatorTypes VALUES(17,'recipient');
INSERT INTO creatorTypes VALUES(18,'director');
INSERT INTO creatorTypes VALUES(19,'scriptwriter');
INSERT INTO creatorTypes VALUES(20,'producer');
INSERT INTO creatorTypes VALUES(21,'interviewee');
INSERT INTO creatorTypes VALUES(22,'interviewer');
INSERT INTO creatorTypes VALUES(23,'cartographer');
INSERT INTO creatorTypes VALUES(24,'inventor');
INSERT INTO creatorTypes VALUES(25,'attorneyAgent');
INSERT INTO creatorTypes VALUES(26,'podcaster');
INSERT INTO creatorTypes VALUES(27,'guest');
INSERT INTO creatorTypes VALUES(28,'presenter');
INSERT INTO creatorTypes VALUES(29,'castMember');


CREATE TABLE itemTypes (
    itemTypeID INTEGER PRIMARY KEY,
    typeName TEXT,
    templateItemTypeID INT,
    display INT DEFAULT 1 -- 0 == hide, 1 == display, 2 == primary
);
INSERT INTO itemTypes VALUES (1,'note',NULL,0);
INSERT INTO itemTypes VALUES (2,'book',NULL,2);
INSERT INTO itemTypes VALUES (3,'bookSection',2,2);
INSERT INTO itemTypes VALUES (4,'journalArticle',NULL,2);
INSERT INTO itemTypes VALUES (5,'magazineArticle',NULL,2);
INSERT INTO itemTypes VALUES (6,'newspaperArticle',NULL,2);
INSERT INTO itemTypes VALUES (7,'thesis',NULL,1);
INSERT INTO itemTypes VALUES (8,'letter',NULL,1);


CREATE TABLE libraries (
    libraryID INTEGER PRIMARY KEY,
    type TEXT NOT NULL,
    editable INT NOT NULL,
    filesEditable INT NOT NULL,
    version INT NOT NULL DEFAULT 0,
    storageVersion INT NOT NULL DEFAULT 0,
    lastSync INT NOT NULL DEFAULT 0,
    archived INT NOT NULL DEFAULT 0
);
INSERT INTO libraries VALUES (1,'user',1,1,131,131,0,0);

-- Primary data applicable to all items
CREATE TABLE items (
    itemID INTEGER PRIMARY KEY,
    itemTypeID INT NOT NULL,
    dateAdded TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    dateModified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    clientDateModified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    libraryID INT NOT NULL,
    key TEXT NOT NULL,
    version INT NOT NULL DEFAULT 0,
    synced INT NOT NULL DEFAULT 0,
    UNIQUE (libraryID, key),
    FOREIGN KEY (libraryID) REFERENCES libraries(libraryID) ON DELETE CASCADE
);
CREATE INDEX items_synced ON items(synced);

CREATE TABLE itemDataValues (
    valueID INTEGER PRIMARY KEY,
    value UNIQUE
);

-- Type-specific data for individual items
CREATE TABLE itemData (
    itemID INT,
    fieldID INT,
    valueID,
    PRIMARY KEY (itemID, fieldID),
    FOREIGN KEY (itemID) REFERENCES items(itemID) ON DELETE CASCADE,
    FOREIGN KEY (fieldID) REFERENCES fieldsCombined(fieldID),
    FOREIGN KEY (valueID) REFERENCES itemDataValues(valueID)
);
CREATE INDEX itemData_fieldID ON itemData(fieldID);


CREATE TABLE creators (
    creatorID INTEGER PRIMARY KEY,
    firstName TEXT,
    lastName TEXT,
    fieldMode INT,
    UNIQUE (lastName, firstName, fieldMode)
);

CREATE TABLE itemCreators (
    itemID INT NOT NULL,
    creatorID INT NOT NULL,
    creatorTypeID INT NOT NULL DEFAULT 1,
    orderIndex INT NOT NULL DEFAULT 0,
    PRIMARY KEY (itemID, creatorID, creatorTypeID, orderIndex),
    UNIQUE (itemID, orderIndex),
    FOREIGN KEY (itemID) REFERENCES items(itemID) ON DELETE CASCADE,
    FOREIGN KEY (creatorID) REFERENCES creators(creatorID) ON DELETE CASCADE,
    FOREIGN KEY (creatorTypeID) REFERENCES creatorTypes(creatorTypeID)
);
CREATE INDEX itemCreators_creatorTypeID ON itemCreators(creatorTypeID);

--
-- Mock data
--
INSERT INTO items (itemID, itemTypeID, libraryID, key) VALUES (1, 1, 1, 'ABCDEFGH');
INSERT INTO itemDataValues VALUES (1, 'A sample title');
INSERT INTO itemData (itemID, fieldID, valueID) VALUES (1, 1, 1); -- title
INSERT INTO itemDataValues VALUES (2, 'Abstract for a sample article');
INSERT INTO itemData (itemID, fieldID, valueID) VALUES (1, 2, 2); -- abstractNote
INSERT INTO itemDataValues VALUES (3, '2022-05-15');
INSERT INTO itemData (itemID, fieldID, valueID) VALUES (1, 6, 3); -- date
INSERT INTO creators VALUES (1, 'Jan', 'Kowalski', 0);
INSERT INTO itemCreators VALUES (1, 1, 8, 0); -- author
