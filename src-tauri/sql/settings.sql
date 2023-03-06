-- name: default_settings!
-- Sets the default settings
-- # Parameters
-- param: minecraft_dir: &str - The directory where Minecraft is installed
INSERT INTO settings (minecraft_dir) VALUES (:minecraft_dir)