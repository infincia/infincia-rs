CREATE TABLE files
(
    id INTEGER DEFAULT nextval('files_id_seq'::regclass) PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    mime_type VARCHAR NOT NULL,
    date TIMESTAMP NOT NULL,
    description VARCHAR,
    sha256 VARCHAR NOT NULL,
    md5 VARCHAR NOT NULL,
    data BYTEA NOT NULL,
    preview BYTEA,
    length BIGINT NOT NULL,
    has_preview BOOLEAN NOT NULL
);
CREATE UNIQUE INDEX files_name_key ON files (name);
CREATE TABLE posts
(
    id INTEGER DEFAULT nextval('posts_id_seq'::regclass) PRIMARY KEY NOT NULL,
    url VARCHAR NOT NULL,
    tags VARCHAR NOT NULL,
    tag_list TEXT[] NOT NULL,
    title VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    date TIMESTAMP NOT NULL,
    updated TIMESTAMP,
    published BOOLEAN NOT NULL,
    uuid VARCHAR NOT NULL,
    owner VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    html VARCHAR NOT NULL
);
CREATE UNIQUE INDEX posts_url_key ON posts (url);
CREATE UNIQUE INDEX posts_uuid_key ON posts (uuid);
CREATE TABLE repos
(
    id INTEGER DEFAULT nextval('repos_id_seq'::regclass) PRIMARY KEY NOT NULL,
    html_url VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    description VARCHAR
);
CREATE TABLE users
(
    id INTEGER DEFAULT nextval('users_id_seq'::regclass) PRIMARY KEY NOT NULL,
    email VARCHAR NOT NULL,
    password_hash BYTEA NOT NULL,
    password_salt BYTEA NOT NULL,
    name VARCHAR NOT NULL,
    avatar BYTEA
);
CREATE UNIQUE INDEX users_email_uindex ON users (email);
CREATE UNIQUE INDEX users_password_salt_uindex ON users (password_salt);



