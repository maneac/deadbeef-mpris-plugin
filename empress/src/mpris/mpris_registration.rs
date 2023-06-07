use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use dbus::blocking::LocalConnection;
use dbus_tree::Factory;

use crate::deadbeef;

use super::{change_signals::SigHandler, media_player::MediaPlayer, player::Player};

pub struct MPRIS {
    pub(super) conn: Option<Rc<LocalConnection>>,
    pub(super) sig_handler: Option<SigHandler>,
    pub(super) exit: AtomicBool,
}

impl MPRIS {
    pub const fn uninit() -> Self {
        Self {
            conn: None,
            sig_handler: None,
            exit: AtomicBool::new(false),
        }
    }

    pub fn init(&mut self, name: &str, api: &'static deadbeef::DB_functions_t) {
        let conn = LocalConnection::new_session().unwrap();

        conn.request_name(name, true, true, false).unwrap();
        let f = Factory::new_fn::<()>();

        let tree = f.tree(()).add(
            f.object_path("/org/mpris/MediaPlayer2", ())
                .introspectable()
                .add(MediaPlayer::from_factory(&f))
                .add(Player::from_factory(&f, api)),
        );

        tree.start_receive(&conn);

        let conn_rc = Rc::new(conn);
        self.conn = Some(Rc::clone(&conn_rc));

        self.sig_handler = Some(SigHandler::new(
            Rc::clone(&conn_rc),
            f.signal("PropertiesChanged", ()),
            api,
        ));
    }

    pub fn handle_event(&self, id: u32, ctx: usize, p1: u32, p2: u32) {
        self.sig_handler
            .as_ref()
            .unwrap()
            .handle_event(id, ctx, p1, p2)
    }

    pub fn listen(&self) {
        while !self.exit.load(Ordering::SeqCst) {
            self.conn
                .as_ref()
                .unwrap()
                .process(Duration::from_millis(100))
                .unwrap();
        }
    }

    pub fn exit(&mut self) {
        println!("exit called");
        self.exit.store(true, Ordering::SeqCst)
    }
}
