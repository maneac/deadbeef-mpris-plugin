#![deny(clippy::all)]
use std::ffi::CString;

use empress::{deadbeef, mpris::MPRIS};

const NO: i8 = 1;

static mut LIST_FILTER: deadbeef::DB_plugin_t = deadbeef::DB_plugin_t {
    type_: 0,
    version_major: 0,
    version_minor: 1,

    id: &NO,
    name: &NO,
    descr: &NO,
    copyright: &NO,
    website: &NO,

    configdialog: &NO,

    api_vmajor: 0,
    api_vminor: 0,
    flags: 0,
    reserved1: 0,
    reserved2: 0,
    reserved3: 0,

    command: None,
    connect: None,
    disconnect: None,
    exec_cmdline: None,
    get_actions: None,
    message: None,
    start: None,
    stop: None,
};

static mut API: Option<&deadbeef::DB_functions_t> = None;

#[no_mangle]
// Note: the name here _must_ match the name of the final
// library file. This assumes that the DeaDBeeF plugin folder
// will contain a `mpris.so` file.
unsafe extern "C" fn mpris_load(
    api: *const deadbeef::DB_functions_t,
) -> *const deadbeef::DB_plugin_t {
    let dialog = CString::new(
        r#"property "Enable" checkbox ddb_mpris.checked 0;
"#,
    )
    .unwrap();

    let name = CString::new("MPRIS").unwrap();
    let id = CString::new("mpris").unwrap();

    let description = CString::new(
        "Registers DeaDBeeF as a MPRIS media player, to enable GNOME media key support",
    )
    .unwrap();

    let website = CString::new("http://github.com/maneac/deadbeef-mpris-plugin").unwrap();
    let copyright = CString::new(include_str!("../../LICENSE")).unwrap();

    LIST_FILTER = deadbeef::DB_plugin_t {
        type_: deadbeef::DB_PLUGIN_MISC as i32,
        version_major: 1,
        version_minor: 0,

        id: id.into_raw(),
        name: name.into_raw(),
        descr: description.into_raw(),
        copyright: copyright.into_raw(),
        website: website.into_raw(),

        configdialog: dialog.into_raw(),

        api_vmajor: 1,
        api_vminor: 15,
        flags: 0,
        reserved1: 0,
        reserved2: 0,
        reserved3: 0,

        command: None,
        connect: None,
        disconnect: None,
        exec_cmdline: None,
        get_actions: None,
        message: Some(handle_message),
        start: Some(start),
        stop: Some(stop),
    };

    API = api.as_ref();

    &LIST_FILTER
}

static mut EMPRESS: MPRIS = MPRIS::uninit();

#[no_mangle]
unsafe extern "C" fn start() -> i32 {
    EMPRESS.init("org.mpris.MediaPlayer2.DeaDBeeF", API.unwrap());
    std::thread::spawn(move || EMPRESS.listen());
    0
}

#[no_mangle]
unsafe extern "C" fn stop() -> i32 {
    EMPRESS.exit();
    0
}

#[no_mangle]
unsafe extern "C" fn handle_message(id: u32, ctx: usize, p1: u32, p2: u32) -> i32 {
    EMPRESS.handle_event(id, ctx, p1, p2);
    0
}
