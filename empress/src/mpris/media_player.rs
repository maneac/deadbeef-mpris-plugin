use std::{rc::Rc, sync::Arc};

use dbus::{
    arg::{Iter, IterAppend},
    MethodErr,
};
use dbus_tree::{
    Access, DataType, Factory, Interface, MTFn, MethodInfo, MethodResult, MethodType, PropInfo,
};

pub(super) struct MediaPlayer {}

/// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html
impl MediaPlayer {
    pub(super) fn from_factory<M, D>(f: &Factory<MTFn>) -> Arc<Interface<M, D>>
    where
        D: DataType,
        M: MethodType<D>,
        std::sync::Arc<dbus_tree::Interface<M, D>>: From<dbus_tree::Interface<MTFn, ()>>,
    {
        let s = Rc::new(Self {});

        let mut interface = f.interface("org.mpris.MediaPlayer2", ());

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("Raise", (), move |m| rc.raise(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_m(f.method("Quit", (), move |m| rc.quit(m)));

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanQuit", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_quit(i, m)),
        );

        let rc = Rc::clone(&s);
        let set_rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("Fullscreen", ())
                .access(Access::ReadWrite)
                .on_get(move |i, m| rc.get_fullscreen(i, m))
                .on_set(move |i, m| set_rc.set_fullscreen(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanSetFullscreen", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_set_fullscreen(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("CanRaise", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_can_raise(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<bool, _>("HasTrackList", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_has_track_list(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<String, _>("Identity", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_identity(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<String, _>("DesktopEntry", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_desktop_entry(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<Vec<String>, _>("SupportedUriSchemes", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_supported_uri_schemes(i, m)),
        );

        let rc = Rc::clone(&s);
        interface = interface.add_p(
            f.property::<Vec<String>, _>("SupportedMimeTypes", ())
                .access(Access::Read)
                .on_get(move |i, m| rc.get_supported_mime_types(i, m)),
        );

        interface.into()
    }
}

// Methods
impl MediaPlayer {
    /// Raise() -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Method:Raise
    fn raise(&self, m: &MethodInfo<MTFn, ()>) -> MethodResult {
        println!("Raise called: {:?}", m);
        Ok(vec![])
    }

    /// Quit() -> nothing
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Method:Quit
    fn quit(&self, m: &MethodInfo<MTFn, ()>) -> MethodResult {
        println!("Quit called: {:?}", m);
        Ok(vec![])
    }
}

// Properties
impl MediaPlayer {
    /// CanQuit - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:CanQuit
    /// Emits changed signal containing new value
    fn get_can_quit(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        i.append(false);
        Ok(())
    }

    /// Fullscreen - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:Fullscreen
    /// Optional
    /// Emits changed signal containing new value
    fn get_fullscreen(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        i.append(false);
        Ok(())
    }

    /// Fullscreen - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:Fullscreen
    /// Optional
    /// Emits changed signal containing new value
    fn set_fullscreen(&self, _i: &mut Iter, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        Err(MethodErr::failed("cannot set fullscreen property"))
    }

    /// CanSetFullscreen - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:CanSetFullscreen
    /// Optional
    /// Emits changed signal containing new value
    fn get_can_set_fullscreen(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(false);
        Ok(())
    }

    /// CanRaise - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:CanRaise
    /// Emits changed signal containing new value
    fn get_can_raise(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        i.append(false);
        Ok(())
    }

    /// HasTrackList - b
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:HasTrackList
    /// Emits changed signal containing new value
    fn get_has_track_list(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(false);
        Ok(())
    }

    /// Identity - s
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:Identity
    /// Emits changed signal containing new value
    fn get_identity(&self, i: &mut IterAppend, _m: &PropInfo<MTFn, ()>) -> Result<(), MethodErr> {
        i.append("DeaDBeeF");
        Ok(())
    }

    /// DesktopEntry - s
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:DesktopEntry
    /// Optional
    /// Emits changed signal containing new value
    fn get_desktop_entry(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append("");
        Ok(())
    }

    /// SupportedUriSchemes - as
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:SupportedUriSchemes
    /// Emits changed signal containing new value
    fn get_supported_uri_schemes(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(Vec::<String>::new());
        Ok(())
    }

    /// SupportedMimeTypes - as
    /// https://specifications.freedesktop.org/mpris-spec/latest/Media_Player.html#Property:SupportedMimeTypes
    /// Emits changed signal containing new value
    fn get_supported_mime_types(
        &self,
        i: &mut IterAppend,
        _m: &PropInfo<MTFn, ()>,
    ) -> Result<(), MethodErr> {
        i.append(Vec::<String>::new());
        Ok(())
    }
}
