#[macro_export]
macro_rules! sql_impl {
    // Base case with explicit lifetime bounds and where clauses
    {
        $ident:ident $(<
            $($lt:lifetime,)*
            $($generic:ident $(: $($generic_bound:tt)+)?),*
            $(,)?
        >)?;
        $apply_filter:ident ($apply_self:ident, $builder:ident) $apply_block:block
        $should_apply_filter:ident($should_self:ident) $should_apply_block:block
        $(where $($where_clause:tt)+)?
    } => {
        impl<'args $(, $($lt,)* $($generic),*)?> $crate::traits::SqlFilter<'args>
            for $ident$(<$($lt,)* $($generic),*>)?
        where
            $($($generic: $($($generic_bound)+ +)? 'args,)*)?
            $($($where_clause)+)?
        {
            #[inline]
            fn $apply_filter(self, builder: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>) {
                let filter_impl = |$apply_self: Self, $builder: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>|
                    $apply_block;
                filter_impl(self, builder)
            }

            #[inline]
            fn $should_apply_filter(&self) -> bool {
                let should_apply_impl = |$should_self: &Self| $should_apply_block;
                should_apply_impl(self)
            }
        }
    };

    // Shorthand case for simple SqlFilter bounds
    {
        $ident:ident $(<$($lt:lifetime,)* $($generic:ident),* $(,)?>)?;
        $apply_filter:ident ($apply_self:ident, $builder:ident) $apply_block:block
        $should_apply_filter:ident($should_self:ident) $should_apply_block:block
    } => {
        $crate::traits::sql_impl! {
            $ident$(<$($lt,)* $($generic: $crate::traits::SqlFilter<'args>),*>)?;
            $apply_filter($apply_self, $builder) $apply_block
            $should_apply_filter($should_self) $should_apply_block
            where
        }
    };

    {
        $ident:ident $(<$($lt:lifetime)? $(,)? $($generic:ident),* $(,)?>)?;
        $apply_filter:ident (_, _) $apply_block:block
        $should_apply_filter:ident(_) $should_apply_block:block
    } => {
        impl<'args, $($($lt,)? $($generic),*)?> $crate::traits::SqlFilter<'args> for $ident$(<$($lt,)? $($generic),*>)?
        $(where
            $($generic: $crate::traits::SqlFilter<'args> + 'args),*)?
        {
            #[inline]
            fn $apply_filter(self, builder: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>) {
                let filter_impl = |_: Self, _: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>| $apply_block;
                filter_impl(self, builder);
            }

            #[inline]
            fn $should_apply_filter(&self) -> bool {
                let should_apply_impl = |_: &Self| $should_apply_block;
                should_apply_impl(self)
            }
        }
    };
}

#[macro_export]
macro_rules! sql_delimiter {
    {
        $(#[$struct_meta:meta])*
        $vis:vis struct $ident:ident $(<
            $($lt:lifetime,)*
            $($generic:ident $(: $($generic_bound:tt)+)?),*
            $(,)?
        >)? {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_ident:ident: $field_ty:ty
            ),* $(,)?
        }

        $apply_filter:ident ($apply_self:ident, $builder:ident) $apply_block:block
        $should_apply_filter:ident($should_self:ident) $should_apply_block:block
        $(where $($where_clause:tt)+)?
    } => {
        $(#[$struct_meta])*
        $vis struct $ident$(<$($lt,)* $($generic),*>)? {
            $(
                $(#[$field_meta])*
                $field_vis $field_ident: $field_ty
            ),*
        }

        impl$(<$($lt,)* $($generic),*>)? $ident$(<$($lt,)* $($generic),*>)?
        $(where $($where_clause)+)?
        {
            #[inline]
            $vis fn new($($field_ident: $field_ty),*) -> Self {
                Self { $($field_ident),* }
            }
        }

        $crate::sql_impl! {
            $ident$(<$($lt,)* $($generic),*>)?;
            $apply_filter($apply_self, $builder) $apply_block
            $should_apply_filter($should_self) $should_apply_block
            $(where
                $($generic: $crate::traits::SqlFilter<'args>),*)?
        }
    };
}

#[macro_export]
macro_rules! sql_operator {
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

        $crate::sql_impl! {
            $ident<T>;

            apply_filter(s, builder) {
                if let Some(val) = s.value {
                    builder.push(s.column);
                    builder.push(concat!(" ", $lit, " "));
                    builder.push_bind(val);
                }
            }

            should_apply_filter(s) {
                s.value.is_some()
            }

            where
                T: ::sqlx::Type<::sqlx::Postgres> + ::sqlx::Encode<'args, ::sqlx::Postgres>
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

        $crate::sql_impl! {
            $ident;

            apply_filter(s, builder) {
                if let Some(value) = s.value {
                    builder.push(s.column);
                    builder.push(concat!(" ", $lit, " "));
                    builder.push_bind(value);
                }
            }

            should_apply_filter(s) {
                s.value.is_some()
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

        $crate::sql_impl! {
            $ident<T>;

            apply_filter(s, builder) {
                if !s.should_apply_filter()  {
                    return;
                }

                builder.push(s.column);
                builder.push(concat!(" ", $lit, " ("));

                let mut first = true;
                for val in s.values {
                    if !first {
                        builder.push(", ");
                    }
                    builder.push_bind(val);
                    first = false;
                }

                builder.push(")");
            }

            should_apply_filter(s) {
                !s.values.is_empty()
            }

            where
                T: ::sqlx::Type<::sqlx::Postgres> + ::sqlx::Encode<'args, ::sqlx::Postgres>
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

    ($vis:vis $ident:ident) => {
        $vis struct $ident;

        impl $ident {
            #[inline]
            $vis fn new() -> Self {
                Self
            }
        }

        $crate::sql_impl! {
            $ident;
            apply_filter(_, _) {}
            should_apply_filter(_) { false }
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
