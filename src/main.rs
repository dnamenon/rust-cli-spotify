use rand::Rng;

use serde_json::json;

use structopt::StructOpt;

extern crate rspotify;
use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util::get_token;
use rspotify::senum::{Country, SearchType};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[structopt(
    about = "Pass a genre and get a random song from that genre"
)]
struct Cli {
    /// The pattern to look for
    genre: String,


}

fn main() {
    let args = Cli::from_args();
    let track = search_spotify(args.genre);


    println!("{}", track);

   
    
}

fn search_spotify(s: String) -> String {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut rng = rand::thread_rng();
            let random: u32 = rng.gen_range(0..100);

            let track_query = format!("genre:{}", s);
            let result = spotify.search(
                &track_query[..],
                SearchType::Track,
                1,
                random,
                Some(Country::UnitedStates),
                None,
            );

            match result {
                Ok(tracks) => {
                    let json = json!(tracks);
                    let tracklink =
                        format!("{}", json["tracks"]["items"][0]["external_urls"]["spotify"]);
                    let artist = format!("{}", json["tracks"]["items"][0]["artists"][0]["name"]);

                    if tracklink == "null" {
                        return String::from("tracks in that genre couldn't be found");
                    }

                    return format!("a {} track: {}, primary artist: {}", s, tracklink, artist);
                }
                Err(err) => return format!("search error!{:?}", err),
            }
        }
        None => return format!("auth failed + {:?}", oauth),
    };
}

