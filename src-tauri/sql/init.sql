-- name: create_tables&
-- Creates all tables needed for the database if they don't exist
BEGIN;
CREATE TABLE IF NOT EXISTS modpacks (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(63) UNIQUE NOT NULL,
    slug VARCHAR(63) UNIQUE NOT NULL,
    premade BOOLEAN NOT NULL DEFAULT FALSE,
    removed BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS modpack_versions (
    id INTEGER PRIMARY KEY NOT NULL,
    modpack_id INTEGER NOT NULL,
    game_version VARCHAR(63) NOT NULL,
    installed BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (modpack_id) REFERENCES modpacks(id)
);

CREATE TABLE IF NOT EXISTS modpack_mods (
    id INTEGER PRIMARY KEY NOT NULL,
    modpack_id INTEGER NOT NULL,
    mod_id INTEGER NOT NULL,
    FOREIGN KEY (modpack_id) REFERENCES modpacks(id),
    FOREIGN KEY (mod_id) REFERENCES mods(id)
);

CREATE TABLE IF NOT EXISTS mods (
    id INTEGER PRIMARY KEY NOT NULL,
    project_id INTEGER NOT NULL,
    name VARCHAR(63) NOT NULL,
    slug VARCHAR(63) NOT NULL,
    page_url VARCHAR(63) NOT NULL
);

CREATE TABLE IF NOT EXISTS mod_versions (
    id INTEGER PRIMARY KEY NOT NULL,
    mod_id INTEGER NOT NULL,
    modpack_version_id INTEGER NOT NULL,
    version_id INTEGER NOT NULL,
    game_version VARCHAR(63) NOT NULL,
    download_url VARCHAR(63) NOT NULL,
    installed BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (mod_id) REFERENCES mods(id),
    FOREIGN KEY (modpack_version_id) REFERENCES modpack_versions(id)
);

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY NOT NULL,
    minecraft_dir VARCHAR(63) NOT NULL,
    stable_only BOOLEAN NOT NULL DEFAULT TRUE
);
COMMIT;
/