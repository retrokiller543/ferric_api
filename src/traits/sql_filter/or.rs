use crate::traits::SqlFilter;
use sqlx::{Postgres, QueryBuilder};

pub struct Or<L, R> {
    pub left: L,
    pub right: R,
}

impl<'args, L: SqlFilter<'args> + 'args, R: SqlFilter<'args> + 'args> Or<L, R> {
    #[inline]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<'args, L: SqlFilter<'args> + 'args, R: SqlFilter<'args> + 'args> SqlFilter<'args>
    for Or<L, R>
{
    #[inline]
    fn apply_filter(self, builder: &mut QueryBuilder<'args, Postgres>) {
        match (
            self.left.should_apply_filter(),
            self.right.should_apply_filter(),
        ) {
            (true, true) => {
                self.left.apply_filter(builder);
                builder.push(" OR ");
                self.right.apply_filter(builder);
            }
            (true, false) => {
                self.left.apply_filter(builder);
            }
            (false, true) => {
                self.right.apply_filter(builder);
            }
            (false, false) => {
                // Do nothing
            }
        }
    }

    #[inline]
    fn should_apply_filter(&self) -> bool {
        match (
            self.left.should_apply_filter(),
            self.right.should_apply_filter(),
        ) {
            (true, true) => true,
            (true, false) => true,
            (false, true) => true,
            (false, false) => false,
        }
    }
}
