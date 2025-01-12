use crate::traits::SqlFilter;
use sqlx::{Postgres, QueryBuilder};

pub struct Not<T> {
    pub(crate) inner: T,
}

impl<'args, T: 'args> Not<T> {
    pub fn new(expr: T) -> Self {
        Self { inner: expr }
    }
}

impl<'args, T: SqlFilter<'args> + 'args> SqlFilter<'args> for Not<T> {
    #[inline]
    fn apply_filter(self, builder: &mut QueryBuilder<'args, Postgres>) {
        if self.should_apply_filter() {
            builder.push("NOT ");
            self.inner.apply_filter(builder);
        }
    }

    #[inline]
    fn should_apply_filter(&self) -> bool {
        self.inner.should_apply_filter()
    }
}
