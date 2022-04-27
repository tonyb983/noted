-- sqlite
-- Data table
DROP TABLE IF EXISTS notes;
DROP TABLE IF EXISTS notes_change_log;

CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    title TEXT NOT NULL DEFAULT '',
    -- Changed from 'content' because sqlite fts5 uses content as a keyword
    body TEXT NOT NULL DEFAULT '',
    tags TEXT NOT NULL DEFAULT ''
);

CREATE VIRTUAL TABLE notes_fts USING fts5(title, body, tags, content=notes, content_rowid=id);

-- Change log table
CREATE TABLE notes_change_log (
    id INTEGER PRIMARY KEY,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    action TEXT,
    table_name TEXT,
    obj_id INTEGER,
    oldvals TEXT
);

-- Update Trigger
-- List all required fields after 'OF' except the LastUpdate field to prevent infinite loop
CREATE TRIGGER notes_update_field 
AFTER UPDATE OF title, body, tags ON notes
BEGIN
  UPDATE notes SET updated=CURRENT_TIMESTAMP WHERE id=NEW.id;
END;

-- Insert Trigger
CREATE TRIGGER notes_track_insert
AFTER INSERT ON notes
BEGIN
  INSERT INTO notes_change_log (action, table_name, obj_id)
  VALUES ('INSERT', 'notes', NEW.id);
END;

-- Update Trigger
CREATE TRIGGER notes_track_update
AFTER UPDATE ON notes
BEGIN
  INSERT INTO notes_change_log (action, table_name, obj_id, oldvals)
  SELECT
    'UPDATE', 'notes', OLD.id, changes
  FROM
    (SELECT
      json_group_object(col, oldval) AS changes
    FROM
      (SELECT
        json_extract(value, '$[0]') as col,
        json_extract(value, '$[1]') as oldval,
        json_extract(value, '$[2]') as newval
      FROM
        json_each(
          json_array(
            json_array('id', OLD.id, NEW.id),
            json_array('created', OLD.created, NEW.created),
            json_array('updated', OLD.updated, NEW.updated),
            json_array('title', OLD.title, NEW.title),
            json_array('body', OLD.body, NEW.body),
            json_array('tags', OLD.tags, NEW.tags)
          )
        )
      WHERE oldval IS NOT newval
      )
    );
END;

-- Delete Trigger
CREATE TRIGGER notes_track_delete
AFTER DELETE ON notes
BEGIN
  INSERT INTO notes_change_log (action, table_name, obj_id, oldvals)
  SELECT
    'DELETE', 'notes', OLD.id, changes
  FROM
    (SELECT
      json_group_object(col, oldval) AS changes
    FROM
      (SELECT
        json_extract(value, '$[0]') as col,
        json_extract(value, '$[1]') as oldval,
        json_extract(value, '$[2]') as newval
      FROM
        json_each(
          json_array(
            json_array('id', OLD.id, null),
            json_array('created', OLD.created, null),
            json_array('updated', OLD.updated, null),
            json_array('title', OLD.title, null),
            json_array('body', OLD.body, null),
            json_array('tags', OLD.tags, null)
          )
        )
      WHERE oldval IS NOT newval
      )
    );
END;