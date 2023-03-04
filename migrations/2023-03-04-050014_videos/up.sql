CREATE TABLE videos (
        ID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        title character varying(255) NOT NULL,
        description text NOT NULL,
        removed boolean NOT NULL
    ); 