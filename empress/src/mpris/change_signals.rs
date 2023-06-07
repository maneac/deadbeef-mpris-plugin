use std::rc::Rc;

use crate::{deadbeef, mpris::player::Lock};
use dbus::{
    arg::{Array, PropMap, Variant},
    blocking::LocalConnection,
    channel::Sender,
    strings::Interface,
    Path,
};
use dbus_tree::Signal;

pub(super) struct SigHandler {
    conn: Rc<LocalConnection>,
    sig: Signal<()>,
    api: &'static deadbeef::DB_functions_t,
}

impl SigHandler {
    pub fn new(
        conn: Rc<LocalConnection>,
        sig: Signal<()>,
        api: &'static deadbeef::DB_functions_t,
    ) -> Self {
        Self { conn, sig, api }
    }
}

impl SigHandler {
    pub fn handle_event(&self, id: u32, ctx: usize, p1: u32, p2: u32) {
        match id {
            deadbeef::DB_EV_NEXT => {
                println!("DB_EV_NEXT: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PREV => {
                println!("DB_EV_PREV: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PLAY_CURRENT => {
                println!("DB_EV_PLAY_CURRENT: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PLAY_NUM => {
                println!("DB_EV_PLAY_NUM: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_STOP => {
                println!("DB_EV_STOP: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PAUSE => {
                println!("DB_EV_PAUSE: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PLAY_RANDOM => {
                println!("DB_EV_PLAY_RANDOM: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_TERMINATE => {
                println!("DB_EV_TERMINATE: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PLAYLIST_REFRESH => {
                println!("DB_EV_PLAYLIST_REFRESH: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_REINIT_SOUND => {
                println!("DB_EV_REINIT_SOUND: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_CONFIGCHANGED => {
                println!("DB_EV_CONFIGCHANGED: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_TOGGLE_PAUSE => {
                println!("DB_EV_TOGGLE_PAUSE: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_ACTIVATED => {
                println!("DB_EV_ACTIVATED: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PAUSED => {
                println!("DB_EV_PAUSED: {}, {}, {}", ctx, p1, p2);
                let state = if p1 > 0 { "Paused" } else { "Playing" };
                self.change_playback_status(state);
            }
            deadbeef::DB_EV_PLAYLISTCHANGED => {
                println!("DB_EV_PLAYLISTCHANGED: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_VOLUMECHANGED => {
                println!("DB_EV_VOLUMECHANGED: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_OUTPUTCHANGED => {
                println!("DB_EV_OUTPUTCHANGED: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PLAYLISTSWITCHED => {
                println!("DB_EV_PLAYLISTSWITCHED: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_SEEK => {
                println!("DB_EV_SEEK: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_ACTIONSCHANGED => {
                println!("DB_EV_ACTIONSCHANGED: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_DSPCHAINCHANGED => {
                println!("DB_EV_DSPCHAINCHANGED: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_SELCHANGED => {
                println!("DB_EV_FOCUS_SELECTION: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_PLUGINSLOADED => {
                println!("DB_EV_FOCUS_SELECTION: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_FOCUS_SELECTION => {
                println!("DB_EV_FOCUS_SELECTION: {}, {}, {}", ctx, p1, p2);
            }
            deadbeef::DB_EV_SONGCHANGED => {
                let event =
                    unsafe { std::ptr::read(ctx as *const deadbeef::ddb_event_trackchange_t) };
                self.change_metadata(event.to as usize).unwrap();
                println!("song changed: {:?}, {}, {}", event.to as usize, p1, p2);
            }
            deadbeef::DB_EV_SONGSTARTED => {
                let event = unsafe { std::ptr::read(ctx as *const deadbeef::ddb_event_track_t) };
                self.change_playback_status("Playing");
                println!("song started: {:?}, {}, {}", event, p1, p2);
            }
            deadbeef::DB_EV_SONGFINISHED => {
                let event = unsafe { std::ptr::read(ctx as *const deadbeef::ddb_event_track_t) };
                println!("song finished: {:?}, {}, {}", event, p1, p2);
            }
            deadbeef::DB_EV_TRACKINFOCHANGED => {
                let event = unsafe { std::ptr::read(ctx as *const deadbeef::ddb_event_track_t) };
                println!(
                    "track info changed: {:?}, {}, {}",
                    event.track as usize, p1, p2
                );
            }
            deadbeef::DB_EV_SEEKED => {
                let event = unsafe { std::ptr::read(ctx as *const deadbeef::ddb_event_playpos_t) };
                println!("track seeked changed: {:?}, {}, {}", event, p1, p2);
            }
            deadbeef::DB_EV_CURSOR_MOVED => {
                let event = unsafe { std::ptr::read(ctx as *const deadbeef::ddb_event_track_t) };
                println!("cursor moved: {:?}, {}, {}", event, p1, p2);
            }
            _ => {
                panic!(
                    "received unknown message: id: {}, ctx: {}, p1: {}, p2: {}",
                    id, ctx, p1, p2
                );
            }
        }
    }

    fn change_playback_status(&self, state: &str) {
        let mut props = PropMap::new();
        props.insert(
            "PlaybackStatus".to_owned(),
            Variant(Box::new(state.to_owned())),
        );

        self.conn
            .send(
                self.sig
                    .msg(
                        &Path::from_slice("/org/mpris/MediaPlayer2").unwrap(),
                        &Interface::new("org.freedesktop.DBus.Properties".to_string()).unwrap(),
                    )
                    .append3(
                        "org.mpris.MediaPlayer2.Player",
                        props,
                        Array::new(Vec::<String>::new()),
                    ),
            )
            .unwrap();
    }

    fn change_metadata(&self, track_id: usize) -> Result<(), String> {
        let mut metadata = PropMap::new();

        metadata.insert(
            "mpris:trackid".to_string(),
            Variant(Box::new(format!(
                "/org/mpris/MediaPlayer2/tracks/{}",
                track_id
            ))),
        );

        unsafe {
            let get_track_fn = self
                .api
                .streamer_get_playing_track
                .ok_or_else(|| "could not get playing track".to_owned())?;

            let track = get_track_fn();

            if track.is_null() {
                return Ok(());
            }

            let _l = Lock::new(self.api);

            let get_meta_fn = self
                .api
                .pl_get_metadata_head
                .ok_or_else(|| "could not get metadata head for track".to_string())?;

            let mut meta = get_meta_fn(track)
                .as_ref()
                .ok_or_else(|| "null metadata returned".to_string())?;

            loop {
                let key = std::ffi::CStr::from_ptr(meta.key).to_str().unwrap();
                let val = std::ffi::CStr::from_ptr(meta.value).to_str().unwrap();

                match key.to_lowercase().as_str() {
                    "artist" => {
                        metadata.insert(
                            "xesam:artist".to_string(),
                            Variant(Box::new(val.to_string())),
                        );
                    }
                    "album artist" => {
                        metadata.insert(
                            "xesam:albumArtist".to_string(),
                            Variant(Box::new(val.to_string())),
                        );
                    }
                    "album" => {
                        metadata.insert(
                            "xesam:album".to_string(),
                            Variant(Box::new(val.to_string())),
                        );
                    }
                    "title" => {
                        metadata.insert(
                            "xesam:title".to_string(),
                            Variant(Box::new(val.to_string())),
                        );
                    }
                    ":uri" => {
                        let (path, file_uri) = if val.starts_with("file://") {
                            (
                                std::path::Path::new(val.strip_prefix("file://").unwrap()),
                                val.to_string(),
                            )
                        } else {
                            (std::path::Path::new(val), format!("file://{}", val))
                        };

                        metadata
                            .insert("xesam:url".to_string(), Variant(Box::new(file_uri.clone())));

                        let art_uri = album_art_from_file(path);

                        println!("art uri: {:?}", &art_uri);
                        if let Some(uri) = art_uri {
                            metadata.insert("mpris:artUrl".to_string(), Variant(Box::new(uri)));
                        };
                    }
                    ":duration" => {
                        let dur = val
                            .split(':')
                            .rev()
                            .enumerate()
                            .fold(0i64, |acc, (idx, v)| {
                                acc + ((60 * idx as i64) * v.parse::<i64>().unwrap())
                            })
                            * 1000
                            * 1000;

                        metadata.insert("mpris:length".to_string(), Variant(Box::new(dur)));
                    }
                    _ => {}
                };

                println!("Key: {}, Val: {}", key, val);
                if meta.next.is_null() {
                    break;
                }
                meta = meta.next.as_ref().unwrap();
            }
        }

        let mut props = PropMap::new();
        props.insert("Metadata".to_owned(), Variant(Box::new(metadata)));
        self.conn
            .send(
                self.sig
                    .msg(
                        &Path::from_slice("/org/mpris/MediaPlayer2").unwrap(),
                        &Interface::new("org.freedesktop.DBus.Properties".to_string()).unwrap(),
                    )
                    .append3(
                        "org.mpris.MediaPlayer2.Player",
                        props,
                        Array::new(Vec::<String>::new()),
                    ),
            )
            .unwrap();

        Ok(())
    }
}

fn album_art_from_file(file_path: &std::path::Path) -> Option<String> {
    let dir = match file_path.parent() {
        Some(d) => d,
        None => return None,
    };
    std::fs::read_dir(dir).unwrap().find_map(|f| {
        if let Ok(file) = f {
            match file.file_name().to_ascii_lowercase().to_str().unwrap() {
                "folder.jpg" | "folder.png" => {
                    Some(format!("file://{}", file.path().to_str().unwrap()))
                }
                _ => None,
            }
        } else {
            None
        }
    })
}
