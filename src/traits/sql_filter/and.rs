use super::sql_delimiter;

// Example usage:
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

/*pub struct And<L, R> {
    pub left: L,
    pub right: R,
}

impl<'args, L: SqlFilter<'args> + 'args, R: SqlFilter<'args> + 'args> And<L, R> {
    #[inline]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<'args, L: SqlFilter<'args> + 'args, R: SqlFilter<'args> + 'args> SqlFilter<'args>
    for And<L, R>
{
    #[inline]
    fn apply_filter(self, builder: &mut QueryBuilder<'args, Postgres>) {
        match (
            self.left.should_apply_filter(),
            self.right.should_apply_filter(),
        ) {
            (true, true) => {
                self.left.apply_filter(builder);
                builder.push(" AND ");
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
*/
