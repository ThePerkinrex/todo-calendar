-- Add up migration script here
CREATE TABLE course (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
	color TEXT NOT NULL
);

CREATE TABLE deadline_category (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
	color TEXT NOT NULL
);

CREATE TABLE deadline (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
	timestamp DATETIME NOT NULL,
	course INTEGER NOT NULL,
	category INTEGER NOT NULL,
	FOREIGN KEY(course) REFERENCES course(id),
	FOREIGN KEY(category) REFERENCES deadline_category(id)
);

CREATE TABLE event_category (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
	color TEXT NOT NULL
);

CREATE TABLE event (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
	start DATETIME NOT NULL,
	end DATETIME NOT NULL,
	course INTEGER NOT NULL,
	category INTEGER NOT NULL,
	FOREIGN KEY(course) REFERENCES course(id),
	FOREIGN KEY(category) REFERENCES event_category(id)
);

INSERT INTO deadline_category(name, color) VALUES('Assignment', 'F94144');
INSERT INTO event_category(name, color) VALUES('Lecture', 'F9C74F');
INSERT INTO event_category(name, color) VALUES('Presentation', 'F3722C');
INSERT INTO event_category(name, color) VALUES('Exam', '43AA8B');
