use super::*;

#[test]
fn steam_hero_url_targets_the_cloudflare_cdn_path() {

    assert_eq!(steam_hero_url(1144200), "https://cdn.cloudflare.steamstatic.com/steam/apps/1144200/library_hero.jpg");

}

#[test]
fn steam_logo_url_targets_the_cloudflare_cdn_path() {

    assert_eq!(steam_logo_url(1144200), "https://cdn.cloudflare.steamstatic.com/steam/apps/1144200/logo_2x.png");

}

#[test]
fn steam_cover_url_targets_the_cloudflare_cdn_path() {

    assert_eq!(steam_cover_url(1144200), "https://cdn.cloudflare.steamstatic.com/steam/apps/1144200/library_600x900_2x.jpg");

}

#[test]
fn resolve_hero_is_empty_without_a_manifest() {

    assert_eq!(resolve_hero(None), "");

}

#[test]
fn resolve_cover_is_empty_without_a_manifest() {

    assert_eq!(resolve_cover(None), "");

}

#[test]
fn first_existing_returns_empty_string_for_an_empty_candidate_list() {

    assert_eq!(first_existing(&[]), "");

}

#[test]
fn to_game_assets_copies_every_resolved_field_onto_the_appid() {

    let resolved = ResolvedAssets {
        hero_url: "https://example.com/hero.jpg".to_string(),
        logo_url: "https://example.com/logo.png".to_string(),
        cover_url: "https://example.com/cover.jpg".to_string(),
    };

    let assets = to_game_assets(4656000, &resolved);

    assert_eq!(assets.appid, 4656000);

    assert_eq!(assets.hero_url, resolved.hero_url);

    assert_eq!(assets.logo_url, resolved.logo_url);

    assert_eq!(assets.cover_url, resolved.cover_url);

}

#[test]
#[ignore]
fn live_resolves_real_art_and_every_resolved_url_returns_200() {

    let appids = [4656000u32, 4279630, 221100, 1466060, 3908940, 4412320];

    for appid in appids {

        let manifest = crate::store_items::fetch_asset_manifest(appid);

        let hero = resolve_hero(manifest.as_ref());

        let cover = resolve_cover(manifest.as_ref());

        let logo = resolve_logo(appid);

        for (kind, url) in [("hero", &hero), ("cover", &cover), ("logo", &logo)] {

            if url.is_empty() {

                println!("appid {} {} -> EMPTY", appid, kind);

                continue;

            }

            let status = ureq::head(url).call().map(|response| response.status()).unwrap_or(0);

            println!("appid {} {} -> {} {}", appid, kind, status, url);

            assert_eq!(status, 200, "appid {} {} did not return 200: {}", appid, kind, url);

        }

    }

}
