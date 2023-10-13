CREATE TABLE IF NOT EXISTS publisher (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS book (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    published DATE NOT NULL,
    publisher_id UUID REFERENCES publisher (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    summary TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS author (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS book_author (
    book_id UUID NOT NULL,
    author_id UUID NOT NULL,
    PRIMARY KEY (book_id, author_id),
    FOREIGN KEY (book_id) REFERENCES book (id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES author (id) ON UPDATE CASCADE ON DELETE CASCADE
);