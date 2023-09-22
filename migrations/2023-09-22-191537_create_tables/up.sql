-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    moderator BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE connection_infos (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    ssh VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    valid BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TYPE request_status_enum AS ENUM ('created', 'processed', 'completed', 'canceled', 'deleted');

CREATE TABLE requests (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    moderator_id INTEGER REFERENCES users(id),
    connection_info INTEGER NOT NULL REFERENCES connection_infos(id),
    status request_status_enum NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    canceled_at TIMESTAMP,
    deleted_at TIMESTAMP,
    processed_at TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE TABLE softwares (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    version VARCHAR NOT NULL,
    description TEXT NOT NULL,
    logo BYTEA,
    source VARCHAR NOT NULL,
    active BOOLEAN NOT NULL,
    installation_script TEXT,
    deletion_script TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TYPE soft_status_enum AS ENUM ('in_queue', 'auto', 'manual', 'completed', 'failed');

CREATE TABLE requests_softwares (
    id SERIAL PRIMARY KEY,
    software_id INTEGER NOT NULL REFERENCES softwares(id),
    request_id INTEGER NOT NULL REFERENCES requests(id),
    to_install BOOLEAN NOT NULL,
    port INTEGER NOT NULL,
    port_valid BOOLEAN,
    status soft_status_enum NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,

    UNIQUE (software_id, request_id)
);

CREATE TABLE softwares_tags (
    id SERIAL PRIMARY KEY,
    software_id INTEGER NOT NULL REFERENCES softwares(id),
    tag_id INTEGER NOT NULL REFERENCES tags(id),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

INSERT INTO users (username, password, moderator, created_at, updated_at)
VALUES ('user1', 'password1', false, NOW(), NOW()),
('user2', 'password2', false, NOW(), NOW()),
('user3', 'password3', true, NOW(), NOW()),
('user4', 'password4', false, NOW(), NOW()),
('user5', 'password5', false, NOW(), NOW()),
('user6', 'password6', true, NOW(), NOW()),
('user7', 'password7', false, NOW(), NOW()),
('user8', 'password8', false, NOW(), NOW()),
('user9', 'password9', true, NOW(), NOW()),
('user10', 'password10', false, NOW(), NOW());

INSERT INTO connection_infos (user_id, ssh, password, valid, created_at, updated_at)
VALUES (1, 'ssh1', 'pass1', true, NOW(), NOW()),
(2, 'ssh2', 'pass2', false, NOW(), NOW()),
(3, 'ssh3', 'pass3', true, NOW(), NOW()),
(4, 'ssh4', 'pass4', false, NOW(), NOW()),
(5, 'ssh5', 'pass5', true, NOW(), NOW()),
(6, 'ssh6', 'pass6', false, NOW(), NOW()),
(7, 'ssh7', 'pass7', true, NOW(), NOW()),
(8, 'ssh8', 'pass8', false, NOW(), NOW()),
(9, 'ssh9', 'pass9', true, NOW(), NOW()),
(10, 'ssh10', 'pass10', false, NOW(), NOW());

INSERT INTO requests (user_id, moderator_id, connection_info, status, created_at, updated_at)
VALUES (1, 3, 1, 'created', NOW(), NOW()),
(2, null, 2, 'processed', NOW(), NOW()),
(3, 6, 3, 'completed', NOW(), NOW()),
(4, null, 4, 'canceled', NOW(), NOW()),
(5, 9, 5, 'deleted', NOW(), NOW()),
(6, 3, 1, 'created', NOW(), NOW()),
(7, null, 2, 'processed', NOW(), NOW()),
(8, 6, 3, 'completed', NOW(), NOW()),
(9, null, 4, 'canceled', NOW(), NOW()),
(10, 9, 5, 'deleted', NOW(), NOW());

INSERT INTO softwares (name, version, description, source, active, created_at, updated_at)
VALUES ('software1', '1.0', 'desc1', 'src1', true, NOW(), NOW()),
('software2', '2.0', 'desc2', 'src2', false, NOW(), NOW()),
('software3', '3.0', 'desc3', 'src3', true, NOW(), NOW()),
('software4', '4.0', 'desc4', 'src4', false, NOW(), NOW()),
('software5', '5.0', 'desc5', 'src5', true, NOW(), NOW()),
('software6', '6.0', 'desc6', 'src6', false, NOW(), NOW()),
('software7', '7.0', 'desc7', 'src7', true, NOW(), NOW()),
('software8', '8.0', 'desc8', 'src8', false, NOW(), NOW()),
('software9', '9.0', 'desc9', 'src9', true, NOW(), NOW()),
('software10', '10.0', 'desc10', 'src10', false, NOW(), NOW());

INSERT INTO tags (name, created_at, updated_at)
VALUES ('tag1', NOW(), NOW()),
('tag2', NOW(), NOW()),
('tag3', NOW(), NOW()),
('tag4', NOW(), NOW()),
('tag5', NOW(), NOW()),
('tag6', NOW(), NOW()),
('tag7', NOW(), NOW()),
('tag8', NOW(), NOW()),
('tag9', NOW(), NOW()),
('tag10', NOW(), NOW());

INSERT INTO requests_softwares (software_id, request_id, to_install, port, status, created_at, updated_at)
VALUES (1, 1, true, 8080, 'in_queue', NOW(), NOW()),
   (2, 2, false, 8081, 'auto', NOW(), NOW()),
   (3, 3, true, 8082, 'manual', NOW(), NOW()),
   (4, 4, false, 8083, 'completed', NOW(), NOW()),
   (5, 5, true, 8084, 'failed', NOW(), NOW()),
   (6, 6, false, 8085, 'in_queue', NOW(), NOW()),
   (7, 7, true, 8086, 'auto', NOW(), NOW()),
   (8, 8, false, 8087, 'manual', NOW(), NOW()),
   (9, 9, true, 8088, 'completed', NOW(), NOW()),
   (10, 10, false, 8089, 'failed', NOW(), NOW());
   
INSERT INTO softwares_tags (software_id, tag_id, created_at, updated_at)
VALUES (1, 1, NOW(), NOW()),
(2, 2, NOW(), NOW()),
(3, 3, NOW(), NOW()),
(4, 4, NOW(), NOW()),
(5, 5, NOW(), NOW()),
(6, 6, NOW(), NOW()),
(7, 7, NOW(), NOW()),
(8, 8, NOW(), NOW()),
(9, 9, NOW(), NOW()),
(10, 10, NOW(), NOW());