//! Filter trait

use and::And;
use not::Not;
use or::Or;
use sqlx::{Postgres, QueryBuilder};

mod_def! {
    pub mod and;
}

mod_def! {
    pub mod or;
}

mod_def! {
    pub mod not;
}

mod_def! {
    pub mod equals;
}

mod_def! {
    pub mod like;
}

mod_def! {
    pub mod r#in;
}

mod_def! {
    pub mod raw;
}

mod_def! {
    pub mod not_equal;
}

mod_def! {
    pub mod greater_than;
}

mod_def! {
    pub mod less_than;
}

mod_def! {
    pub mod greater_than_or_equal;
}

mod_def! {
    pub mod less_than_or_equal;
}

mod_def! {
    pub mod i_like;
}

mod_def! {
    pub mod not_in;
}

macro_rules! sql_operator {
    // ─────────────────────────────────────────────────────────────────────────────
    // Case 1: e.g. `Equals<T>` with a single "lit" operator (like =, >, etc.)
    // ─────────────────────────────────────────────────────────────────────────────
    ($vis:vis $ident:ident, $lit:literal) => {
        $vis struct $ident<T> {
            column: &'static str,
            value: Option<T>,
        }

        impl<'args, T> $ident<T>
        where
            T: ::sqlx::Type<::sqlx::Postgres> + ::sqlx::Encode<'args, ::sqlx::Postgres> + 'args,
        {
            #[inline]
            $vis fn new(column: &'static str, value: Option<T>) -> Self {
                Self { column, value }
            }
        }

        impl $ident<$crate::traits::sql_filter::Raw> {
            #[inline]
            $vis fn new_raw(column: &'static str, value: $crate::traits::sql_filter::Raw) -> Self {
                Self { column, value: Some(value) }
            }
        }

        impl<'args, T> $crate::traits::SqlFilter<'args> for $ident<T>
        where
            T: ::sqlx::Type<::sqlx::Postgres> + ::sqlx::Encode<'args, ::sqlx::Postgres> + 'args,
        {
            #[inline]
            fn apply_filter(self, builder: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>) {
                if let Some(val) = self.value {
                    builder.push(self.column);
                    builder.push(concat!(" ", $lit, " "));
                    builder.push_bind(val);
                }
            }

            #[inline]
            fn should_apply_filter(&self) -> bool {
                self.value.is_some()
            }
        }

        impl<'args> $crate::traits::SqlFilter<'args> for $ident<$crate::traits::sql_filter::Raw>
        {
            #[inline]
            fn apply_filter(self, builder: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>) {
                if let Some(val) = self.value {
                    builder.push(self.column);
                    builder.push(concat!(" ", $lit, " "));
                    val.apply_filter(builder)
                }

            }

            #[inline]
            fn should_apply_filter(&self) -> bool {
                self.value.is_some()
            }
        }

        ::paste::paste! {
            #[inline]
            pub fn [< $ident:snake >]<'args, T>(
                column: &'static str,
                value: Option<T>
            ) -> $crate::traits::sql_filter::Filter<$ident<T>>
            where
                T: ::sqlx::Type<::sqlx::Postgres> + ::sqlx::Encode<'args, ::sqlx::Postgres> + 'args,
            {
                $crate::traits::sql_filter::Filter::new($ident::new(column, value))
            }

            #[inline]
            pub fn [< $ident:snake _raw >]<'args>(
                column: &'static str,
                value: $crate::traits::sql_filter::Raw
            ) -> $crate::traits::sql_filter::Filter<$ident<$crate::traits::sql_filter::Raw>> {
                $crate::traits::sql_filter::Filter::new($ident::new_raw(column, value))
            }
        }
    };

    // ─────────────────────────────────────────────────────────────────────────────
    // Case 2: e.g. `NotEqual<T>` that has a typed value
    //     => $ident<$ty:ty>, $lit:literal
    // ─────────────────────────────────────────────────────────────────────────────
    ($vis:vis $ident:ident<$ty:ty>, $lit:literal) => {
        $vis struct $ident {
            column: &'static str,
            value: Option<$ty>,
        }

        impl $ident {
            #[inline]
            $vis fn new(column: &'static str, value: Option<impl Into<$ty>>) -> Self {
                let value = value.map(::core::convert::Into::into);
                Self { column, value }
            }
        }

        impl<'args> $crate::traits::SqlFilter<'args> for $ident {
            #[inline]
            fn apply_filter(self, builder: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>) {
                if let Some(value) = self.value {
                    builder.push(self.column);
                    builder.push(concat!(" ", $lit, " "));
                    builder.push_bind(value);
                }
            }

            #[inline]
            fn should_apply_filter(&self) -> bool {
                self.value.is_some()
            }
        }

        ::paste::paste! {
            #[inline]
            pub fn [< $ident:snake >]<'args>(
                column: &'static str,
                value: Option<impl Into<$ty>>
            ) -> $crate::traits::sql_filter::Filter<$ident> {
                $crate::traits::sql_filter::Filter::new($ident::new(column, value))
            }

            #[inline]
            pub fn [< $ident:snake _raw >]<'args>(
                column: &'static str,
                value: $crate::traits::sql_filter::Raw
            ) -> $crate::traits::sql_filter::Filter<$ident> {
                $crate::traits::sql_filter::Filter::new($ident::new(column, Some(value.0)))
            }
        }
    };

    // ─────────────────────────────────────────────────────────────────────────────
    // Case 3: e.g. `In<T>` with a list of T => $ident[]
    // ─────────────────────────────────────────────────────────────────────────────
    ($vis:vis $ident:ident[], $lit:literal) => {
        $vis struct $ident<T> {
            column: &'static str,
            values: Vec<T>,
        }

        impl<'args, T> $ident<T>
        where
            T: ::sqlx::Type<::sqlx::Postgres> + ::sqlx::Encode<'args, ::sqlx::Postgres> + 'args,
        {
            #[inline]
            $vis fn new(column: &'static str, values: impl IntoIterator<Item = T>) -> Self {
                let values = values.into_iter().collect();
                Self { column, values }
            }
        }

        impl<'args, T> $crate::traits::SqlFilter<'args> for $ident<T>
        where
            T: ::sqlx::Type<::sqlx::Postgres> + ::sqlx::Encode<'args, ::sqlx::Postgres> + 'args,
        {
            #[inline]
            fn apply_filter(self, builder: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>)  {
                if !self.should_apply_filter()  {
                    return;
                }

                builder.push(self.column);
                builder.push(concat!(" ", $lit, " ("));

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

            #[inline]
            fn should_apply_filter(&self) -> bool {
                !self.values.is_empty()
            }
        }

        ::paste::paste! {
            #[inline]
            pub fn [< $ident:snake >]<'args, T>(
                column: &'static str,
                values: impl IntoIterator<Item = T>
            ) -> $crate::traits::sql_filter::Filter<$ident<T>>
            where
                T: ::sqlx::Type<::sqlx::Postgres> + ::sqlx::Encode<'args, ::sqlx::Postgres> + 'args,
            {
                $crate::traits::sql_filter::Filter::new($ident::new(column, values))
            }
        }
    };

    // ─────────────────────────────────────────────────────────────────────────────
    // Case 4: a "no-op" or purely logical operator with no fields => $ident
    // ─────────────────────────────────────────────────────────────────────────────
    ($vis:vis $ident:ident) => {
        $vis struct $ident;

        impl $ident {
            #[inline]
            $vis fn new() -> Self {
                Self
            }
        }

        impl<'args> $crate::traits::SqlFilter<'args> for $ident {
            #[inline]
            fn apply_filter(self, _: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>) {
                // no-op
            }

            #[inline]
            fn should_apply_filter(&self) -> bool {
                false
            }
        }

        ::paste::paste! {
            #[inline]
            pub fn [< $ident:snake >]() -> $crate::traits::sql_filter::Filter<$ident> {
                $crate::traits::sql_filter::Filter::new($ident::new())
            }

            #[inline]
            pub fn [< $ident:snake _raw >]() -> $crate::traits::sql_filter::Filter<$ident> {
                $crate::traits::sql_filter::Filter::new($ident::new())
            }
        }
    };
}

sql_operator!(pub NoOpFilter);
use crate::mod_def;
pub(crate) use sql_operator;

pub trait SqlFilter<'args> {
    fn apply_filter(self, builder: &mut QueryBuilder<'args, Postgres>);
    fn should_apply_filter(&self) -> bool;
}

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
