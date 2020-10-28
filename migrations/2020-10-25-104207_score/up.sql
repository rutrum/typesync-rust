CREATE TABLE scores (
    name VARCHAR(255) NOT NULL,
    genius_id VARCHAR(15) NOT NULL,
    milliseconds BIGINT NOT NULL,
    absolute_time BIGINT NOT NULL,
    mode TINYINT NOT NULL,
    PRIMARY KEY (name, genius_id, milliseconds, absolute_time, mode)
);
