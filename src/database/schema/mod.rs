#![allow(clippy::module_name_repetitions)]

use yoke::Yoke;

pub mod commit;
pub mod prefixes;
pub mod repository;
pub mod tag;
pub mod tree;

pub type Yoked<T> = Yoke<T, Box<[u8]>>;

pub const SCHEMA_VERSION: &str = "5";
