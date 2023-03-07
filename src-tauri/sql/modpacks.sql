-- name: create_modpack ->
-- Creates a modpack
-- # Parameters
-- param: name: &str - The name of the modpack
-- param: slug: &str - The slug of the modpack
-- param: premade: &bool - Whether the modpack is premade
INSERT INTO modpacks (name, slug, premade) VALUES ( :name, :slug, :premade )
RETURNING id