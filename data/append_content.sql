-- sqlite3
-- Append text to the end of the 'content' column.
UPDATE notes SET content = content || ' ' || 'Here is some more text for the first note.' WHERE id = 1;