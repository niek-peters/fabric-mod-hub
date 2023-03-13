WITH RECURSIVE dependency_hierarchy AS (
    SELECT id, mod_id, version_id, game_version, download_url, dependency_of FROM mod_versions WHERE dependency_of IS NULL AND id = ?1

    UNION ALL

    SELECT mv.id, mv.mod_id, mv.version_id, mv.game_version, mv.download_url, mv.dependency_of FROM mod_versions mv, dependency_hierarchy WHERE mv.dependency_of = dependency_hierarchy.id
)
SELECT * FROM dependency_hierarchy;