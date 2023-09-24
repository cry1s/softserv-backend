-- This file should undo anything in `up.sql`
DROP TABLE requests_softwares;
DROP TABLE softwares_tags;
DROP TABLE tags;
DROP TABLE softwares;
DROP TABLE requests;
DROP TABLE connection_infos;
DROP TABLE users;
DROP TYPE request_status_enum;
DROP TYPE soft_status_enum;