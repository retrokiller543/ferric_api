mod_def! {
    pub mod filter;
}

use crate::{mod_def, sql_delimiter, sql_operator};

sql_delimiter! {
    pub struct And<L, R> {
        pub left: L,
        pub right: R
    }

    apply_filter(s, builder) {
        match (
            s.left.should_apply_filter(),
            s.right.should_apply_filter(),
        ) {
            (true, true) => {
                s.left.apply_filter(builder);
                builder.push(" AND ");
                s.right.apply_filter(builder);
            }
            (true, false) => {
                s.left.apply_filter(builder);
            }
            (false, true) => {
                s.right.apply_filter(builder);
            }
            (false, false) => {}
        }
    }

    should_apply_filter(s) {
        s.left.should_apply_filter() || s.right.should_apply_filter()
    }
}

sql_delimiter! {
    pub struct Or<L, R> {
        pub left: L,
        pub right: R
    }

    apply_filter(s, builder) {
        match (
            s.left.should_apply_filter(),
            s.right.should_apply_filter(),
        ) {
            (true, true) => {
                s.left.apply_filter(builder);
                builder.push(" OR ");
                s.right.apply_filter(builder);
            }
            (true, false) => {
                s.left.apply_filter(builder);
            }
            (false, true) => {
                s.right.apply_filter(builder);
            }
            (false, false) => {}
        }
    }

    should_apply_filter(s) {
        s.left.should_apply_filter() || s.right.should_apply_filter()
    }
}

sql_delimiter! {
    pub struct Not<T> {
        pub inner: T,
    }

    apply_filter(s, builder) {
        if s.should_apply_filter() {
            builder.push("NOT ");
            s.inner.apply_filter(builder);
        }
    }

    should_apply_filter(s) {
        s.inner.should_apply_filter()
    }
}

sql_operator!(pub Equals, "=");
sql_operator!(pub Like<String>, "LIKE");
sql_operator!(pub InValues[], "IN");
sql_operator!(pub NotEquals, "!=");
sql_operator!(pub GreaterThan, ">");
sql_operator!(pub LessThan, "<");
sql_operator!(pub GreaterThanOrEqual, ">=");
sql_operator!(pub LessThanOrEqual, "<=");
sql_operator!(pub ILike<String>, "ILIKE");
sql_operator!(pub NotInValues[], "NOT IN");
sql_operator!(pub NoOpFilter);
