-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    avatar VARCHAR NOT NULL,
    moderator BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TYPE request_status_enum AS ENUM ('created', 'processed', 'completed', 'canceled', 'deleted');

CREATE TABLE requests (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    moderator_id INTEGER REFERENCES users(id),
    status request_status_enum NOT NULL,
    ssh_address VARCHAR,
    ssh_password VARCHAR,
    created_at TIMESTAMP NOT NULL,
    processed_at TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE TABLE softwares (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    logo VARCHAR,
    active BOOLEAN NOT NULL,
    name VARCHAR NOT NULL UNIQUE,
    version VARCHAR NOT NULL,
    source VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,

    UNIQUE (name, version)
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TYPE soft_status_enum AS ENUM ('new', 'processed', 'completed', 'failed', 'canceled');

CREATE TABLE requests_softwares (
    software_id INTEGER NOT NULL REFERENCES softwares(id),
    request_id INTEGER NOT NULL REFERENCES requests(id),
    to_install BOOLEAN NOT NULL,
    status soft_status_enum NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    PRIMARY KEY (software_id, request_id)
);

CREATE TABLE softwares_tags (
    software_id INTEGER NOT NULL REFERENCES softwares(id),
    tag_id INTEGER NOT NULL REFERENCES tags(id),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    PRIMARY KEY (software_id, tag_id)
);
