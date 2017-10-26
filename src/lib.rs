#[macro_use]
extern crate stdweb;

mod diff;
pub use diff::Context;

mod node;
pub use node::{Child, Node, NodeBuilder};

mod path;

pub mod widget;

use std::borrow::Cow;

pub type Str = Cow<'static, str>;
