use crate::traits::SqlFilter;
use sqlx::{Postgres, QueryBuilder};

/// UNSAFE AF!!!!
pub struct Raw(pub &'static str);

impl<'args> SqlFilter<'args> for Raw {
    #[inline]
    fn apply_filter(self, builder: &mut QueryBuilder<'args, Postgres>) {
        if self.should_apply_filter() {
            builder.push(self.0);
        }
    }

    #[inline]
    fn should_apply_filter(&self) -> bool {
        !self.0.is_empty()
    }
}
