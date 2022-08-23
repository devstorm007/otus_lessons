mod client;
mod server;
mod tests;

/*

mod server {
mod database {
    struct Db { ... }
        // ...
    }
    mod protocol {
        enum Message { ... }
        // ...
    }
    struct Server { ... }
}
mod client {
    struct Client { ... }
}

self - current path
self::database::Db
self::protocol::Message
self::Server

crate / my_lib / super - root path
crate::client::Client
my_lib::client::Client
super::client::Client


src/
❏ lib.rs / main.rs
❏ server
    ❏ database.rs
    ❏ protocol.rs
    ❏ mod.rs
❏ client.rs

// lib.rs
mod server;
mod client;
// server/mod.rs
mod database;
mod protocol;
// client.rs
struct Client;

OR

src/
❏ lib.rs / main.rs
❏ server
    ❏ database.rs
    ❏ protocol.rs
❏ server.rs
❏ client.rs

// lib.rs
mod server;
mod client;
// server.rs
mod database;
mod protocol;
// client.rs
struct Client;

use **************************************************************************
pub mod storage {
    ...
}
mod client {
    use crate::storage::{store, remove};
    fn handle_request(req: Request) {
        match req {
            Request::Store(item) => store(item).unwrap(),
            Request::Remove(id) => remove(id).unwrap(),
        }
    }
}

 use crate::path::to::module;
 use module::Struct;
 use module::{Struct, AnotherStruct, func, Trait};
 use module::{some_func, submodule::another_func};
 use module::{self, some_func};
 use module::*;
 use module::item as SomeAlias;
 use module::Trait as _;

non_exhaustive **************************************************************
//old way
pub enum Mode {
    Read,
    Write,
    #[doc(hidden)]
    __NonExhaustive,
}
match mode {
    Mode::Read => { ... },
    Mode::Write => { ... },
    _ => { ... },
}

#[non_exhaustive]
pub enum Mode {
    Read,
    Write,
}
match mode {
    Mode::Read => { ... },
    Mode::Write => { ... },
    _ => { ... }, // user must add this
}

pub struct  ****************************************************************
pub struct View {
    pub parent: Node,
    pub camera: Camera2d,
    children: Vec<Child>,
}
impl View {
    pub fn new(parent: &Node) -> Self { ... }
}
let view = View::new(&parent);
// let view = View { parent, camera, children };
// let parent = view.children;
let parent = &view.parent;
let View { parent, camera, ..} = view;

pub use ****************************************************************

pub mod useful_stuff {
    pub use internal_stuff::Thing;

    mod internal_stuff {
        pub struct Thing;
        struct InternalThat;
    }
}

 Нет pub – приватно (только текущий и дочерние модули)
 pub – публично (+ родительский модуль, если текущий модуль pub)
 pub(crate) – публично в пределах крейта
 pub(super) – публично в пределах родительского модуля
 pub(in some::path) – публично в пределах указанного модуля
 pub(self) – аналогично отсутствию pub

 use my_crate::prelude::* - all public will be open


sealed trait *************************************************************
mod sealed {
    pub trait Sealed {}
    impl Sealed for i32 {}
}
pub trait MyTrait: sealed::Sealed {
    fn foo() {}
}
impl MyTrait for i32 {
    // ...
}

declarative macros *******************************************************

macro! {} // ошибка
macro_rules! macro {
    () => {}
}
macro! {} // ok


macro! {} // ошибка
mod macro {
    macro_rules! macro {
        () => {}
    }
    macro! {} // ok
}
macro! {} // ошибка


macro! {} // ошибка
#[macro_use]
mod macro {
    macro_rules! macro {
        () => {}
    }
    macro! {} // ok
}
macro! {} // ok


// some_crate/lib.rs
#[macro_export]
macro_rules! macro {
    () => {}
}
// our_code/lib.rs
use some_crate::macro;
// или
#[macro_use]
extern crate some_crate;
*/
