use crate::traits::{And, Not, Or, SqlFilter};
use sqlx::{Postgres, QueryBuilder};

pub struct Filter<T>(T);

impl<T> Filter<T> {
    #[inline]
    pub fn new(filter: T) -> Self {
        Filter(filter)
    }
}

impl<'args, T: 'args> Filter<T>
where
    T: SqlFilter<'args>,
{
    #[inline]
    pub fn and<U>(self, other: U) -> Filter<And<T, U>>
    where
        U: SqlFilter<'args>,
    {
        Filter(And {
            left: self.0,
            right: other,
        })
    }

    #[inline]
    pub fn or<U>(self, other: U) -> Filter<Or<T, U>>
    where
        U: SqlFilter<'args>,
    {
        Filter(Or {
            left: self.0,
            right: other,
        })
    }

    #[inline]
    pub fn not(self) -> Filter<Not<T>> {
        Filter(Not::new(self.0))
    }
}

impl<'args, T> SqlFilter<'args> for Filter<T>
where
    T: SqlFilter<'args>,
{
    #[inline]
    fn apply_filter(self, builder: &mut QueryBuilder<'args, Postgres>) {
        self.0.apply_filter(builder);
    }

    #[inline]
    fn should_apply_filter(&self) -> bool {
        self.0.should_apply_filter()
    }
}
