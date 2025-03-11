-- Add down migration script here
-- 1. Recreate the deadline table without the status column.
CREATE TABLE deadline_old (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    course INTEGER NOT NULL,
    category INTEGER NOT NULL,
    FOREIGN KEY(course) REFERENCES course(id),
    FOREIGN KEY(category) REFERENCES deadline_category(id)
);

-- Copy existing data (ignoring the status column)
INSERT INTO deadline_old (id, name, timestamp, course, category)
SELECT id, name, timestamp, course, category FROM deadline;

-- Replace the modified table.
DROP TABLE deadline;
ALTER TABLE deadline_old RENAME TO deadline;

-- 2. Recreate the event table without the status column.
CREATE TABLE event_old (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    start DATETIME NOT NULL,
    end DATETIME NOT NULL,
    course INTEGER NOT NULL,
    category INTEGER NOT NULL,
    FOREIGN KEY(course) REFERENCES course(id),
    FOREIGN KEY(category) REFERENCES event_category(id)
);

-- Copy existing data (ignoring the status column)
INSERT INTO event_old (id, name, start, end, course, category)
SELECT id, name, start, end, course, category FROM event;

-- Replace the modified table.
DROP TABLE event;
ALTER TABLE event_old RENAME TO event;

-- 3. Drop the status table.
DROP TABLE status;