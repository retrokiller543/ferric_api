//! Filter trait

mod_def! {
    pub mod implementations;
    pub mod raw;
}

use crate::mod_def;
use sqlx::{Postgres, QueryBuilder};

pub trait SqlFilter<'args> {
    fn apply_filter(self, builder: &mut QueryBuilder<'args, Postgres>);
    fn should_apply_filter(&self) -> bool;
}
