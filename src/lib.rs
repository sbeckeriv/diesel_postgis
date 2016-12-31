#[macro_use]
extern crate diesel;

mod types {
    use diesel::types::HasSqlType;
    use diesel::pg::{Pg, PgTypeMetadata};

    #[derive(Clone, Copy)]
    pub struct Point;
    #[derive(Clone, Copy)]
    pub struct Geometry;

    impl HasSqlType<Point> for Pg {
        fn metadata() -> PgTypeMetadata {
            PgTypeMetadata {
                oid: 0,
                array_oid: 0,
            }
        }
    }

    impl HasSqlType<Geometry> for Pg {
        fn metadata() -> PgTypeMetadata {
            PgTypeMetadata {
                oid: 0,
                array_oid: 0,
            }
        }
    }
}

mod functions {
    use types::*;
    use diesel::types::*;
    // sql_function!(length, length_t, (x: TsVector) -> Integer);
    // sql_function!(numnode, numnode_t, (x: TsQuery) -> Integer);
    // sql_function!(plainto_tsquery, plain_to_tsquery_t, (x: Text) -> TsQuery);
    // sql_function!(querytree, querytree_t, (x: TsQuery) -> Text);
    // sql_function!(strip, strip_t, (x: TsVector) -> TsVector);
    // sql_function!(to_tsquery, to_tsquery_t, (x: Text) -> TsQuery);
    // sql_function!(to_tsvector, to_tsvector_t, (x: Text) -> TsVector);
    // sql_function!(ts_headline, ts_headline_t, (x: Text, y: TsQuery) -> Text);
    // sql_function!(ts_rank, ts_rank_t, (x: TsVector, y: TsQuery) -> Float);
    // sql_function!(ts_rank_cd, ts_rank_cd_t, (x: TsVector, y: TsQuery) -> Float);
    //
}

mod dsl {
    use types::*;
    use diesel::expression::{Expression, AsExpression};

    mod predicates {
        use types::*;
        use diesel::query_builder::QueryBuilder;
        // infix_predicate!(Matches, " @@ ");
        // infix_predicate!(Concat, " || ", TsVector);
        // infix_predicate!(And, " && ", TsQuery);
        // infix_predicate!(Or, " || ", TsQuery);
        // infix_predicate!(Contains, " @> ");
        // infix_predicate!(ContainedBy, " @> ");
        //
    }

    use self::predicates::*;
    pub trait PointExtensions: Expression<SqlType = Point> + Sized {
        // fn matches<T: AsExpression<TsQuery>>(self, other: T) -> Matches<Self, T::Expression> {
        // Matches::new(self, other.as_expression())
        // }
        //
        // fn concat<T: AsExpression<TsVector>>(self, other: T) -> Concat<Self, T::Expression> {
        // Concat::new(self, other.as_expression())
        // }
    }
    pub trait GeometryExtensions: Expression<SqlType = Geometry> + Sized {
        // fn matches<T: AsExpression<TsVector>>(self, other: T) -> Matches<Self, T::Expression> {
        // Matches::new(self, other.as_expression())
        // }
        //
        // fn and<T: AsExpression<TsQuery>>(self, other: T) -> And<Self, T::Expression> {
        // And::new(self, other.as_expression())
        // }
        //
        // fn or<T: AsExpression<TsQuery>>(self, other: T) -> Or<Self, T::Expression> {
        // Or::new(self, other.as_expression())
        // }
        //
        // fn contains<T: AsExpression<TsQuery>>(self, other: T) -> Contains<Self, T::Expression> {
        // Contains::new(self, other.as_expression())
        // }
        //
        // fn contained_by<T: AsExpression<TsQuery>>(self,
        // other: T)
        // -> ContainedBy<Self, T::Expression> {
        // ContainedBy::new(self, other.as_expression())
        // }
    }
    //
    impl<T: Expression<SqlType = Point>> PointExtensions for T {}
    impl<T: Expression<SqlType = Geometry>> GeometryExtensions for T {}
    //

}

pub use self::types::*;
pub use self::functions::*;
pub use self::dsl::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
