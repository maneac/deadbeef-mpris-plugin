use crate::deadbeef;
use dbus::{
    arg::{Iter, IterAppend, Variant},
    MethodErr,
};
use dbus_tree::{
    Access, DataType, Factory, Interface, MTFn, MethodInfo, MethodResult, MethodType, PropInfo,
};
use std::{collections::HashMap, rc::Rc, sync::Arc};

type MD = HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>;

pub(super) struct Player {
    api: &'static deadbeef::DB_functions_t,
}

/// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html
impl Player {
    pub(super) fn from_factory<M, D>(
        f: &Factory<MTFn>,
        api: &'static deadbeef::DB_functions_t,
    ) -> Arc<Interface<M, D>>
    where
        D: DataType,
        M: MethodType<D>,
        std::sync::Arc<dbus_tree::Interface<M, D>>: From<dbus_tree::Interface<MTFn, ()>>,
    {
        let s = Rc::new(Self { api });

        let mut interface = f.interface("org.mpris.MediaPlayer2.Player", ());

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("Next", (), move |m| rc.next(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("Previous", (), move |m| rc.previous(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("Pause", (), move |m| rc.pause(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("PlayPause", (), move |m| rc.play_pause(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("Stop", (), move |m| rc.stop(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("Play", (), move |m| rc.play(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("Seek", (), move |m| rc.seek(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("SetPosition", (), move |m| rc.set_position(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("OpenUri", (), move |m| rc.open_uri(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_s({
            rc.seeked();
            f.signal("Seeked", ())
        });

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<String, _>("PlaybackStatus", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_playback_status(i, m)),
        );

        let rc = Rc::clone(&s);
        let set_rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<f64, _>("Rate", ())
                .access(Access::ReadWrite)
                .on_get(move |i, m| rc.get_playback_rate(i, m))
                .on_set(move |i, m| set_rc.set_playback_rate(i, m)),
        );

        let rc = Rc::clone(&s);
        let set_rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("Shuffle", ())
                .access(Access::ReadWrite)
                .on_get(move |i, m| rc.get_shuffle(i, m))
                .on_set(move |i, m| set_rc.set_shuffle(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<MD, _>("Metadata", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_metadata(i, m)),
        );

        let rc = Rc::clone(&s);
        let set_rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<f64, _>("Volume", ())
                .access(Access::ReadWrite)
                .on_get(move |i, m| rc.get_volume(i, m))
                .on_set(move |i, m| set_rc.set_volume(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<i64, _>("Position", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_position(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<f64, _>("MinimumRate", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_minimum_playback_rate(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<f64, _>("MaximumRate", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_maximum_playback_rate(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanGoNext", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_go_next(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanGoPrevious", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_go_previous(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanPlay", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_play(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanPause", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_pause(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanSeek", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_seek(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanControl", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_control(i, m)),
        );

        interface.into()
    }
}

// Methods
impl Player {
    /// Next() -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Next
    fn next(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        unsafe {
            let sendmessage_fn = self
                .api
                .sendmessage
                .ok_or_else(|| MethodErr::failed("unable to get sendmessage function"))?;

            sendmessage_fn(deadbeef::DB_EV_NEXT, 0, 0, 0);
        }
        Ok(vec![])
    }

    /// Previous() -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Previous
    fn previous(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        unsafe {
            let sendmessage_fn = self
                .api
                .sendmessage
                .ok_or_else(|| MethodErr::failed("unable to get sendmessage function"))?;

            sendmessage_fn(deadbeef::DB_EV_PREV, 0, 0, 0);
        }
        Ok(vec![])
    }

    /// Pause() -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Pause
    fn pause(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        unsafe {
            let output_fn = self
                .api
                .get_output
                .ok_or_else(|| MethodErr::failed("unable to get output"))?;

            let output = output_fn()
                .as_ref()
                .ok_or_else(|| MethodErr::failed("null output device returned"))?;

            let pause_fn = output
                .pause
                .ok_or_else(|| MethodErr::failed("unable to pause on output"))?;

            pause_fn();
        }
        unsafe {
            let sendmessage_fn = self
                .api
                .sendmessage
                .ok_or_else(|| MethodErr::failed("unable to get sendmessage function"))?;

            sendmessage_fn(deadbeef::DB_EV_PAUSE, 0, 0, 0);
        }
        Ok(vec![])
    }

    /// PlayPause() -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:PlayPause
    fn play_pause(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        unsafe {
            let sendmessage_fn = self
                .api
                .sendmessage
                .ok_or_else(|| MethodErr::failed("unable to get sendmessage function"))?;

            sendmessage_fn(deadbeef::DB_EV_TOGGLE_PAUSE, 0, 0, 0);
        }
        Ok(vec![])
    }

    /// Stop() -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Stop
    fn stop(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        unsafe {
            let sendmessage_fn = self
                .api
                .sendmessage
                .ok_or_else(|| MethodErr::failed("unable to get sendmessage function"))?;

            sendmessage_fn(deadbeef::DB_EV_STOP, 0, 0, 0);
        }
        Ok(vec![])
    }

    /// Play() -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Play
    fn play(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        unsafe {
            let sendmessage_fn = self
                .api
                .sendmessage
                .ok_or_else(|| MethodErr::failed("unable to get sendmessage function"))?;

            sendmessage_fn(deadbeef::DB_EV_PLAY_CURRENT, 0, 0, 0);
        }
        Ok(vec![])
    }

    /// Seek(x: Offset) -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:Seek
    fn seek(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        Err(MethodErr::failed("CanSeek is permantently false"))
    }

    /// SetPosition(o: TrackId, x: Position) -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:SetPosition
    fn set_position(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        Err(MethodErr::failed("CanSeek is permantently false"))
    }

    /// OpenUri(s: Uri) -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Method:OpenUri
    fn open_uri(&self, _m: &MethodInfo<MTFn, ()>) -> MethodResult {
        Err(MethodErr::failed("OpenURI is unimplemented"))
    }
}

// Signals
impl Player {
    /// Seeked(x: Position)
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Signal:Seeked
    fn seeked(&self) {
        println!("Seeked signal");
    }
}

// Properties
impl Player {
    /// PlaybackStatus - s
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:PlaybackStatus
    /// Emits changed signal containing new value
    fn get_playback_status(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        println!("Get playback status called");

        let playback_state = unsafe {
            let output_fn = self
                .api
                .get_output
                .ok_or_else(|| MethodErr::failed("unable to get output"))?;

            let output = output_fn()
                .as_ref()
                .ok_or_else(|| MethodErr::failed("null output device returned"))?;

            let playback_state_fn = output
                .state
                .ok_or_else(|| MethodErr::failed("unable to get playback state for output"))?;

            playback_state_fn()
        };

        let state = match playback_state {
            deadbeef::DDB_PLAYBACK_STATE_STOPPED => "Stopped",
            deadbeef::DDB_PLAYBACK_STATE_PLAYING => "Playing",
            deadbeef::DDB_PLAYBACK_STATE_PAUSED => "Paused",
            _ => {
                return Err(MethodErr::failed(&format!(
                    "invalid playback state: {}",
                    playback_state
                )))
            }
        };

        println!("Playback status: {}", state);

        i.append(state);

        Ok(())
    }

    // Optional, and hard
    /// LoopStatus - s
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:LoopStatus
    /// Optional
    /// Emits changed signal containing new value
    // fn get_loop_status(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
    //     println!("Loop status called");
    //     Ok(())
    // }

    // Optional, and hard
    /// LoopStatus - s
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:LoopStatus
    /// Optional
    /// Emits changed signal containing new value
    // fn set_loop_status(&self, i: &mut Iter, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
    //     println!("Loop status called");
    //     Ok(())
    // }

    /// Rate - d
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:Rate
    /// Emits changed signal containing new value
    fn get_playback_rate(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(1.0f64);
        Ok(())
    }

    /// Rate - d
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:Rate
    /// Emits changed signal containing new value
    fn set_playback_rate(&self, _i: &mut Iter, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        Ok(())
    }

    /// Shuffle - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:Shuffle
    /// Optional
    /// Emits changed signal containing new value
    fn get_shuffle(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        let shuffled = unsafe {
            let shuffle_fn = self
                .api
                .streamer_get_shuffle
                .ok_or_else(|| MethodErr::failed("could not get shuffled status"))?;
            shuffle_fn() > 0
        };
        i.append(shuffled);
        Ok(())
    }

    /// Shuffle - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:Shuffle
    /// Optional
    /// Emits changed signal containing new value
    fn set_shuffle(&self, _i: &mut Iter, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        Ok(())
    }

    /// Metadata - a{sv}
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:Metadata
    /// Emits changed signal containing new value
    fn get_metadata(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        println!("Metadata called");

        unsafe {
            let get_track_fn = self
                .api
                .streamer_get_playing_track
                .ok_or_else(|| MethodErr::failed("could not get playing track"))?;

            let track = get_track_fn();

            if !track.is_null() {
                let _l = Lock::new(self.api);

                let get_meta_fn = self
                    .api
                    .pl_get_metadata_head
                    .ok_or_else(|| MethodErr::failed("could not get metadata head for track"))?;

                let meta = get_meta_fn(track)
                    .as_ref()
                    .ok_or_else(|| MethodErr::failed("null metadata returned"))?;

                let key = std::ffi::CStr::from_ptr(meta.key);
                let val = std::ffi::CStr::from_ptr(meta.key);

                println!("key: {:#?}\tval: {:#?}", key.to_str(), val.to_str());
            } else {
                let get_track_fn = self
                    .api
                    .streamer_get_current_fileinfo
                    .ok_or_else(|| MethodErr::failed("could not get file info"))?;

                let track = get_track_fn();

                if track.is_null() {
                    println!("TRACK IS NULL");
                    return Ok(());
                }

                println!("readpos: {}", track.as_ref().unwrap().readpos);
            }
        }

        // https://www.freedesktop.org/wiki/Specifications/mpris-spec/metadata
        let mut d = MD::new();

        d.insert(
            String::from("mpris:trackid"),
            Variant(Box::new(String::from(
                "/org/mpris/MediaPlayer2/DeaDBeeF/tracks/one",
            ))),
        );

        d.insert(
            String::from("mpris:artUrl"),
            Variant(Box::new(String::from("file://"))),
        );

        d.insert(
            String::from("xesam:title"),
            Variant(Box::new(String::from("track one"))),
        );

        i.append(d);
        Ok(())
    }

    /// Volume - d
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:Volume
    /// Emits changed signal containing new value
    fn get_volume(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        i.append(1.0f64);
        Ok(())
    }

    /// Volume - d
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:Volume
    /// Emits changed signal containing new value
    fn set_volume(&self, _i: &mut Iter, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        Ok(())
    }

    /// Position - x
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:Position
    /// Emits changed signal containing new value
    fn get_position(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        println!("get position called");
        i.append(0i64);
        Ok(())
    }

    /// MinimumRate - d
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:MinimumRate
    /// Emits changed signal containing new value
    fn get_minimum_playback_rate(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(1.0f64);
        Ok(())
    }

    /// MaximumRate - d
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:MaximumRate
    /// Emits changed signal containing new value
    fn get_maximum_playback_rate(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(1.0f64);
        Ok(())
    }

    /// CanGoNext - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanGoNext
    /// Emits changed signal containing new value
    fn get_can_go_next(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(true);
        Ok(())
    }

    /// CanGoPrevious - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanGoPrevious
    /// Emits changed signal containing new value
    fn get_can_go_previous(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(true);
        Ok(())
    }

    /// CanPlay - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanPlay
    /// Emits changed signal containing new value
    fn get_can_play(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        i.append(true);
        Ok(())
    }

    /// CanPause - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanPause
    /// Emits changed signal containing new value
    fn get_can_pause(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        i.append(true);
        Ok(())
    }

    /// CanSeek - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanSeek
    /// Emits changed signal containing new value
    fn get_can_seek(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        i.append(false);
        Ok(())
    }

    /// CanControl - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Property:CanControl
    /// Emits changed signal containing new value
    fn get_can_control(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(true);
        Ok(())
    }
}

pub(super) struct Lock {
    api: &'static deadbeef::DB_functions_t,
}

impl Lock {
    pub fn new(api: &'static deadbeef::DB_functions_t) -> Self {
        unsafe { (api.pl_lock.unwrap())() };
        Self { api }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        unsafe {
            (self.api.pl_unlock.unwrap())();
        }
    }
}
