use super::sql_delimiter;

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
