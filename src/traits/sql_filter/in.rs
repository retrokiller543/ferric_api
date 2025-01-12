/*use sqlx::{Postgres, QueryBuilder};
use crate::traits::SqlFilter;

pub struct In<T> {
    column: &'static str,
    values: Vec<T>,
}

impl<'args, T: sqlx::Type<Postgres> + sqlx::Encode<'args, Postgres> + 'args> In<T> {
    pub fn new(column: &'static str, values: Vec<T>) -> Self {
        Self { column, values }
    }
}

impl<'args, T: sqlx::Type<Postgres> + sqlx::Encode<'args, Postgres> + 'args> SqlFilter<'args>
    for In<T>
{
    fn apply_filter(self, builder: &mut QueryBuilder<'args, Postgres>) {
        builder.push(self.column);
        builder.push(" IN (");

        let mut first = true;
        for val in self.values {
            if !first {
                builder.push(", ");
            }
            builder.push_bind(val);
            first = false;
        }

        builder.push(")");
    }
}*/
use crate::traits::sql_operator;

sql_operator!(pub InValues[], "IN");
