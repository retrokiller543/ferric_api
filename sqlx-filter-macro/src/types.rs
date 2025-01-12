use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Ident, LitStr, MetaList, Token, Type, TypePath, Visibility, braced, parse_quote};

pub(crate) struct FilterTable {
    pub(crate) meta: Option<MetaList>,
    pub(crate) vis: Option<Visibility>,
    pub(crate) name: Ident,
    pub(crate) sql: FilterSql,
}

impl ToTokens for FilterTable {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            meta,
            vis,
            name,
            sql,
        } = self;

        let fields = sql.expr.fields();
        let token_fields = fields.iter().filter_map(|(name, ty, optional)| {
            if let ColumnVal::Type(ty) = ty {
                if *optional {
                    Some(quote! {#name: Option<#ty>})
                } else {
                    Some(quote! {#name: #ty})
                }
            } else {
                None
            }
        });

        let fields = fields
            .iter()
            .filter(|(_, ty, _)| {
                if let ColumnVal::Type(_) = ty {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        let optional_fields = fields
            .iter()
            .filter(|(_, _, optional)| *optional)
            .collect::<Vec<_>>();

        let optional_field_names = optional_fields.iter().map(|(name, _, _)| quote! {#name});

        let optional_field_builder = optional_fields.iter().map(|(name, ty, _)| {
            if let ColumnVal::Type(ty) = ty {
                quote! {
                    #[inline]
                    #vis fn #name(mut self, #name: impl Into<#ty>) -> Self {
                        self.#name = Some(#name.into());
                        self
                    }
                }
            } else {
                quote! {compile_error!("Found Raw type among fields")}
            }
        });

        let req_fields = fields
            .iter()
            .filter(|(_, _, optional)| !*optional)
            .collect::<Vec<_>>();

        let req_fields_fn_input = req_fields.iter().map(|(name, ty, _)| {
            if let ColumnVal::Type(ty) = ty {
                quote! {#name: impl Into<#ty>}
            } else {
                quote! {compile_error!("Found Raw type among fields")}
            }
        });

        let req_fields_into = req_fields.iter().map(|(name, _, _)| {
            quote! {let #name = #name.into();}
        });

        let req_field_names = req_fields.iter().map(|(name, _, _)| quote! {#name});

        let struct_init = if !req_fields.is_empty() {
            quote! {
                Self {
                    #(#req_field_names),*,
                    #(#optional_field_names: None),*
                }
            }
        } else {
            quote! {
                Self {
                    #(#optional_field_names: None),*
                }
            }
        };

        let struct_def = quote! {
            #meta
            #vis struct #name {
                #(#token_fields,)*
            }

            impl #name {
                #[inline]
                #vis fn new( #(#req_fields_fn_input),* ) -> Self {
                    #(#req_fields_into)*

                    #struct_init
                }

                #(#optional_field_builder)*
            }
        };

        tokens.extend(struct_def);

        let FilterSql {
            /*columns,
            table_name,*/
            expr,
            ..
        } = sql;

        /*let stmt = quote! {
            SELECT #columns FROM #table_name WHERE
        }.to_string();*/

        let expanded = quote! {
            impl<'args> crate::traits::SqlFilter<'args> for #name {
                #[inline]
                fn apply_filter(self, builder: &mut ::sqlx::QueryBuilder<'args, ::sqlx::Postgres>) {
                    #expr.apply_filter(builder);
                }

                #[inline]
                fn should_apply_filter(&self) -> bool {
                    true
                }
            }
        };

        tokens.extend(expanded);
    }
}

impl Parse for FilterTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let meta = input.parse().ok();
        let vis = input.parse().ok();
        input.parse::<Token![struct]>()?;
        let name = input.parse()?;

        let content;
        braced!(content in input);
        let sql = content.parse()?;

        if !input.is_empty() {
            println!(
                "Unexpected tokens after SQL: {}",
                input.fork().parse::<proc_macro2::TokenStream>()?
            );
        }

        Ok(Self {
            meta,
            vis,
            name,
            sql,
        })
    }
}

/// Parses either `*` or `a, b, c as c_example`
pub(crate) enum Columns {
    All,
    Defined(Vec<(String, String)>),
}

impl ToTokens for Columns {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let expanded = match self {
            Columns::All => quote! {"*"},
            Columns::Defined(cols) => {
                let columns = cols
                    .iter()
                    .map(|(name, alias)| quote! {#name as #alias}.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                quote! {#columns}
            }
        };

        tokens.extend(expanded);
    }
}

impl Parse for Columns {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Ok(Columns::All)
        } else {
            let mut columns = Vec::new();
            while !input.is_empty() {
                let name: Ident = input.parse()?;
                let alias = if input.peek(Token![as]) {
                    input.parse::<Token![as]>()?;
                    let alias: Ident = input.parse()?;
                    alias.to_string()
                } else {
                    name.to_string()
                };
                columns.push((name.to_string(), alias));

                if !input.peek(Token![,]) {
                    break;
                }
                input.parse::<Token![,]>()?;
            }
            Ok(Columns::Defined(columns))
        }
    }
}

/// Parses `SELECT * FROM example_table WHERE [conditions]`
#[allow(dead_code)]
pub(crate) struct FilterSql {
    pub(crate) columns: Columns,
    pub(crate) table_name: String,
    pub(crate) expr: Expression,
}

impl Parse for FilterSql {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let select = input.parse::<Ident>()?;

        if select.to_string().to_uppercase().as_str() != "SELECT" {
            return Err(input.error("Expected `SELECT`"));
        }

        let columns = input.parse()?;

        let from = input.parse::<Ident>()?;

        if from.to_string().to_uppercase().as_str() != "FROM" {
            return Err(input.error("Expected `FROM`"));
        }

        let table_name: Ident = input.parse()?;

        let where_ident = input.parse::<Ident>()?;

        if where_ident.to_string().to_uppercase().as_str() != "WHERE" {
            return Err(input.error("Expected `WHERE`"));
        }

        let expr = input.parse()?;

        Ok(FilterSql {
            columns,
            table_name: table_name.to_string(),
            expr,
        })
    }
}

/// Parses `example_col LIKE String`
pub(crate) struct Condition {
    pub(crate) column_name: Ident,
    pub(crate) field_alias: Option<Ident>,
    pub(crate) operator: SqlOperator,
    pub(crate) column_type: ColumnVal,
    pub(crate) optional: bool,
}

impl Condition {
    fn rust_name(&self) -> &Ident {
        if let Some(alias) = &self.field_alias {
            alias
        } else {
            &self.column_name
        }
    }
}

pub(crate) enum ColumnVal {
    Type(Type),
    Raw(LitStr),
}

impl Parse for ColumnVal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            Ok(Self::Raw(input.parse()?))
        } else {
            Ok(Self::Type(input.parse()?))
        }
    }
}

pub(crate) enum Expression {
    Condition(Condition),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

impl Expression {
    fn fields(&self) -> Vec<(&Ident, &ColumnVal, bool)> {
        let mut fields = Vec::new();

        match self {
            Expression::Condition(condition) => fields.push((
                condition.rust_name(),
                &condition.column_type,
                condition.optional,
            )),
            Expression::And(left, right) => {
                fields.extend(left.fields());
                fields.extend(right.fields());
            }
            Expression::Or(left, right) => {
                fields.extend(left.fields());
                fields.extend(right.fields());
            }
            Expression::Not(expr) => fields.extend(expr.fields()),
        }

        fields
    }
}

impl Parse for Expression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Base condition
        let left = Expression::Condition(input.parse()?);

        let op: Option<Ident> = input.parse()?;
        match op.map(|i| i.to_string().to_uppercase()) {
            Some(op) if op == String::from("AND") => {
                let right = Self::parse(input)?;
                Ok(Expression::And(Box::new(left), Box::new(right)))
            }
            Some(op) if op == String::from("OR") => {
                let right = Self::parse(input)?;
                Ok(Expression::Or(Box::new(left), Box::new(right)))
            }
            Some(op) if op == String::from("NOT") => {
                let expr = Self::parse(input)?;
                Ok(Expression::Not(Box::new(expr)))
            }
            None => Ok(left),
            Some(op) => Err(syn::Error::new(op.span(), "Unexpected operator")),
        }
    }
}

impl ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let filter_expr = match self {
            Expression::Condition(c) => {
                let operator = &c.operator;
                let column = &c.column_name;

                if let ColumnVal::Raw(lit) = &c.column_type {
                    let path: TypePath = parse_quote! {#operator};
                    let seg = path.path.segments.last().unwrap();
                    let ident = &seg.ident;
                    let new_ident = format_ident!("{}_raw", ident);

                    quote! { crate::traits::#new_ident(stringify!(#column), crate::traits::Raw(#lit)) }
                } else {
                    let rust_name = c.rust_name();

                    quote! { #operator(stringify!(#column), self.#rust_name) }
                }
            }
            Expression::And(l, r) => quote! { #l.and(#r) },
            Expression::Or(l, r) => quote! { #l.or(#r) },
            Expression::Not(e) => quote! { #e.not() },
        };

        tokens.extend(filter_expr);
    }
}

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let optional = input.peek(Token![?]);
        if optional {
            input.parse::<Token![?]>()?;
        }

        let column_name = input.parse()?;
        let mut field_alias = None;

        let lookahead = input.lookahead1();

        if lookahead.peek(Token![as]) {
            input.parse::<Token![as]>()?;
            field_alias = input.parse()?;
        }

        let operator = input.parse()?;
        let column_type = input.parse()?;

        Ok(Self {
            column_name,
            field_alias,
            operator,
            column_type,
            optional,
        })
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            column_name,
            operator,
            optional,
            ..
        } = self;

        let rust_name = self.rust_name();

        match operator {
            SqlOperator::Equals => quote! {crate::traits::equals},
            /*SqlOperator::NotEquals => {}
            SqlOperator::GreaterThan => {}
            SqlOperator::LessThan => {}
            SqlOperator::GreaterThanOrEqual => {}
            SqlOperator::LessThanOrEqual => {}*/
            SqlOperator::Like => quote! {crate::traits::like},
            /*SqlOperator::ILike => {}*/
            SqlOperator::In => quote! {crate::traits::in_values},
            /*SqlOperator::NotIn => {}*/
            _ => unimplemented!("sadly this has not been done yet :("),
        }
        .to_tokens(tokens);

        if *optional {
            tokens.extend(quote! {
                (stringify!(#column_name), #rust_name)
            })
        } else {
            tokens.extend(quote! {
                (stringify!(#column_name), self.#rust_name)
            })
        }
        /*
        let mut sql = quote! {#column_name #operator}.to_string();

        if let ColumnVal::Raw(raw) = column_type {
            sql.push_str(&raw.value())
        }

        let mut expanded = quote! {
            builder.push(#sql);
        };

        if let ColumnVal::Type(_) = column_type {
            expanded.extend(quote! {builder.push_bind(self.#column_name);})
        }

        tokens.extend(expanded);*/
    }
}

pub(crate) enum SqlOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Like,
    ILike,
    In,
    NotIn,
}

/*impl SqlOperator {
    fn as_str(&self) -> &'static str {
        match self {
            SqlOperator::Equals => "=",
            SqlOperator::NotEquals => "!=",
            SqlOperator::GreaterThan => ">",
            SqlOperator::LessThan => "<",
            SqlOperator::GreaterThanOrEqual => ">=",
            SqlOperator::LessThanOrEqual => "<=",
            SqlOperator::Like => "LIKE",
            SqlOperator::ILike => "ILIKE",
            SqlOperator::In => "IN",
            SqlOperator::NotIn => "NOT IN",
        }
    }
}
*/
impl Parse for SqlOperator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Ok(SqlOperator::Equals)
        } else if lookahead.peek(Token![>]) {
            input.parse::<Token![>]>()?;
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                Ok(SqlOperator::GreaterThanOrEqual)
            } else {
                Ok(SqlOperator::GreaterThan)
            }
        } else if lookahead.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                Ok(SqlOperator::LessThanOrEqual)
            } else {
                Ok(SqlOperator::LessThan)
            }
        } else {
            let op: Ident = input.parse()?;
            match op.to_string().to_uppercase().as_str() {
                "LIKE" => Ok(SqlOperator::Like),
                "ILIKE" => Ok(SqlOperator::ILike),
                "IN" => Ok(SqlOperator::In),
                "NOT" => {
                    if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?;
                        Ok(SqlOperator::NotEquals)
                    } else {
                        let next: Ident = input.parse()?;
                        if next.to_string().to_uppercase() == "IN" {
                            Ok(SqlOperator::NotIn)
                        } else {
                            Err(syn::Error::new(next.span(), "Expected 'IN' after 'NOT'"))
                        }
                    }
                }
                _ => Err(syn::Error::new(op.span(), "Invalid SQL operator")),
            }
        }
    }
}

impl ToTokens for SqlOperator {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SqlOperator::Equals => quote! {crate::traits::equals}.to_tokens(tokens),
            SqlOperator::NotEquals => quote! {crate::traits::not_equal}.to_tokens(tokens),
            SqlOperator::GreaterThan => quote! {crate::traits::greater_than}.to_tokens(tokens),
            SqlOperator::LessThan => quote! {crate::traits::less_than}.to_tokens(tokens),
            SqlOperator::GreaterThanOrEqual => {
                quote! {crate::traits::greater_than_or_equal}.to_tokens(tokens)
            }
            SqlOperator::LessThanOrEqual => {
                quote! {crate::traits::less_than_or_equal}.to_tokens(tokens)
            }
            SqlOperator::Like => quote! {crate::traits::like}.to_tokens(tokens),
            SqlOperator::ILike => quote! {crate::traits::i_like}.to_tokens(tokens),
            SqlOperator::In => quote! {crate::traits::in_values}.to_tokens(tokens),
            SqlOperator::NotIn => quote! {crate::traits::not_in_values}.to_tokens(tokens),
        }
    }
}
