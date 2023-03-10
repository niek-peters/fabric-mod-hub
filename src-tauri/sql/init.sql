-- Creates all tables needed for the database if they don't exist
BEGIN;
CREATE TABLE IF NOT EXISTS modpacks (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(63) UNIQUE NOT NULL,
    slug VARCHAR(63) UNIQUE NOT NULL,
    premade BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS modpack_versions (
    id INTEGER PRIMARY KEY NOT NULL,
    modpack_id INTEGER NOT NULL,
    game_version VARCHAR(63) NOT NULL,
    installed BOOLEAN NOT NULL DEFAULT FALSE,
    loaded BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (modpack_id) REFERENCES modpacks(id) ON DELETE CASCADE,
    CONSTRAINT modpack_version_unique UNIQUE (modpack_id, game_version)
);

CREATE TABLE IF NOT EXISTS modpack_mods (
    id INTEGER PRIMARY KEY NOT NULL,
    modpack_id INTEGER NOT NULL,
    mod_id INTEGER NOT NULL,
    FOREIGN KEY (modpack_id) REFERENCES modpacks(id) ON DELETE CASCADE,
    FOREIGN KEY (mod_id) REFERENCES mods(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS mods (
    id INTEGER PRIMARY KEY NOT NULL,
    project_id VARCHAR(63) UNIQUE NOT NULL,
    name VARCHAR(63) UNIQUE NOT NULL,
    slug VARCHAR(63) UNIQUE NOT NULL,
    page_url VARCHAR(63) NOT NULL
);

CREATE TABLE IF NOT EXISTS mod_versions (
    id INTEGER PRIMARY KEY NOT NULL,
    mod_id INTEGER NOT NULL,
    version_id VARCHAR(63) UNIQUE NOT NULL,
    game_version VARCHAR(63) NOT NULL,
    download_url VARCHAR(63) NOT NULL,
    FOREIGN KEY (mod_id) REFERENCES mods(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY CHECK (id = 0) NOT NULL,
    minecraft_dir VARCHAR(63) NOT NULL,
    stable_only BOOLEAN NOT NULL DEFAULT TRUE
);
COMMIT;
