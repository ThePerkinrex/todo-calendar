-- Add up migration script here

-- 1. Create the new status table.
CREATE TABLE status (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT NOT NULL
);

-- 2. Recreate the deadline table with the new status column and foreign key.
CREATE TABLE deadline_new (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    course INTEGER NOT NULL,
    category INTEGER NOT NULL,
    status INTEGER,  -- New optional column
    FOREIGN KEY(course) REFERENCES course(id),
    FOREIGN KEY(category) REFERENCES deadline_category(id),
    FOREIGN KEY(status) REFERENCES status(id)
);

-- Copy existing data (status will be NULL for all rows)
INSERT INTO deadline_new (id, name, timestamp, course, category)
SELECT id, name, timestamp, course, category FROM deadline;

-- Replace the old table.
DROP TABLE deadline;
ALTER TABLE deadline_new RENAME TO deadline;

-- 3. Recreate the event table with the new status column and foreign key.
CREATE TABLE event_new (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    start DATETIME NOT NULL,
    end DATETIME NOT NULL,
    course INTEGER NOT NULL,
    category INTEGER NOT NULL,
    status INTEGER,  -- New optional column
    FOREIGN KEY(course) REFERENCES course(id),
    FOREIGN KEY(category) REFERENCES event_category(id),
    FOREIGN KEY(status) REFERENCES status(id)
);

-- Copy existing data (status will be NULL for all rows)
INSERT INTO event_new (id, name, start, end, course, category)
SELECT id, name, start, end, course, category FROM event;

-- Replace the old table.
DROP TABLE event;
ALTER TABLE event_new RENAME TO event;

INSERT INTO status (name, color) VALUES ('To Do', '#d9534f');
INSERT INTO status (name, color) VALUES ('In Progress', '#f0ad4e');
INSERT INTO status (name, color) VALUES ('Done', '#5cb85c');
