SELECT mv.id, mv.mod_id, mv.version_id, mv.game_version, mv.download_url, mv.dependency_of FROM mods m INNER JOIN modpack_versions mpv ON mpv.id = ?1 INNER JOIN modpacks mp ON mp.id = mpv.modpack_id INNER JOIN modpack_mods mpm ON mpm.mod_id = m.id INNER JOIN mod_versions mv ON m.id = mv.mod_id WHERE mpm.modpack_id = mp.id;