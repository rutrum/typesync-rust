CREATE TABLE scores (
    name TEXT NOT NULL,
    genius_id TEXT NOT NULL,
    milliseconds BIGINT NOT NULL,
    absolute_time BIGINT NOT NULL,
    mode TEXT NOT NULL,
    PRIMARY KEY (name, genius_id, milliseconds, absolute_time, mode)
);
