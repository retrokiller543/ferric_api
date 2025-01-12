use super::sql_delimiter;

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
