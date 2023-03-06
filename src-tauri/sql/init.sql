-- name: create_table_modpacks?
-- Creates the modpacks table if it doesn't exist
CREATE TABLE IF NOT EXISTS modpacks (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(63) UNIQUE NOT NULL,
    slug VARCHAR(63) UNIQUE NOT NULL,
    premade BOOLEAN NOT NULL DEFAULT FALSE,
    removed BOOLEAN NOT NULL DEFAULT FALSE
);

-- name: create_table_modpack_versions?
-- Creates the modpack_versions table if it doesn't exist
CREATE TABLE IF NOT EXISTS modpack_versions (
    id INTEGER PRIMARY KEY NOT NULL,
    modpack_id INTEGER NOT NULL,
    game_version VARCHAR(63) NOT NULL,
    installed BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (modpack_id) REFERENCES modpacks(id)
);

-- name: create_table_modpack_mods?
-- Creates the modpack_mods table if it doesn't exist
CREATE TABLE IF NOT EXISTS modpack_mods (
    id INTEGER PRIMARY KEY NOT NULL,
    modpack_id INTEGER NOT NULL,
    mod_id INTEGER NOT NULL,
    FOREIGN KEY (modpack_id) REFERENCES modpacks(id),
    FOREIGN KEY (mod_id) REFERENCES mods(id)
);

-- name: create_table_mods?
-- Creates the mods table if it doesn't exist
CREATE TABLE IF NOT EXISTS mods (
    id INTEGER PRIMARY KEY NOT NULL,
    project_id INTEGER NOT NULL,
    name VARCHAR(63) NOT NULL,
    slug VARCHAR(63) NOT NULL,
    page_url VARCHAR(63) NOT NULL
);

-- name: create_table_mod_versions?
-- Creates the mod_versions table if it doesn't exist
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

-- name: create_table_settings?
-- Creates the settings table if it doesn't exist
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY NOT NULL,
    minecraft_dir VARCHAR(63) NOT NULL,
    stable_only BOOLEAN NOT NULL DEFAULT TRUE
);