use std::io::BufRead as _;

use anyhow::{Context as _, Result};

use futures_util::StreamExt;

use chrono::Local;
use config::Config;
use rspotify::{
    model::PlaylistId,
    prelude::{BaseClient, OAuthClient},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    client_id: String,
    client_secret: String,
    playlist_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = dirs::config_dir()
        .context("config_dir not found")?
        .join("spdump")
        .join("config.toml");

    let config = Config::builder()
        .add_source(config::File::with_name(
            config_path
                .to_str()
                .context("failed to convert the config file name into &str")?,
        ))
        .add_source(config::Environment::with_prefix("SP"))
        .build()?
        .try_deserialize::<AppConfig>()?;

    let spo = {
        use rspotify::{AuthCodeSpotify, Credentials, OAuth, prelude::*, scopes};

        let creds = Credentials {
            id: config.client_id,
            secret: Some(config.client_secret),
        };

        let oauth = OAuth {
            redirect_uri: "http://127.0.0.1:8888/callback".into(),
            scopes: scopes!(
                "playlist-read-private playlist-modify-private playlist-modify-public user-library-read"
            ),
            ..Default::default()
        };

        let spo = AuthCodeSpotify::new(creds, oauth);
        spo.prompt_for_token(&spo.get_authorize_url(false)?).await?;

        spo
    };

    let playlist_id = PlaylistId::from_id(&config.playlist_id)?;

    let now_pl = spo
        .playlist_items(playlist_id.clone(), None, None)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    println!(
        "The playlist has {} items. This will replace all items in the playlist. Really proceed? (y/N):",
        now_pl.len(),
    );

    let line = std::io::stdin()
        .lock()
        .lines()
        .next()
        .context("failed to get new line")??;

    if !["yes", "y"].contains(&line.to_lowercase().as_str()) {
        println!("Cancelled");
    }

    let now_pl = now_pl
        .into_iter()
        .filter_map(|t| t.track)
        .filter_map(|t| t.id().map(|i| i.into_static()))
        .collect::<Vec<_>>();

    if now_pl.len() != 0 {
        for chunk in now_pl.chunks(100) {
            let tracks = chunk.into_iter().map(Clone::clone);
            spo.playlist_remove_all_occurrences_of_items(playlist_id.clone(), tracks, None)
                .await?;
        }
    }
    let now = Local::now().date_naive();

    println!("removed all songs in the playlist");

    spo.playlist_change_detail(
        playlist_id.clone(),
        None,
        None,
        Some(&format!("Dumped at {now}")),
        None,
    )
    .await?;

    println!("changed detail");

    let liked = spo
        .current_user_saved_tracks(None)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    println!("total liked songs: {}", liked.len());

    for chunk in liked.chunks(100) {
        let liked = chunk
            .into_iter()
            .map(|c| c.track.clone())
            .filter_map(|t| t.id)
            .map(Into::into);

        spo.playlist_add_items(playlist_id.clone(), liked, None)
            .await?;
    }

    Ok(())
}
