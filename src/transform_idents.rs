use syn::visit_mut::*;
use syn::punctuated::*;
use syn::spanned::Spanned;
use syn::*;
use quote::quote_spanned;

// Notes:
// This visitor traverses a syntax tree and replaces all "leaf identifiers" (e.g. function arguments, but not function names) with an
// expression that gets the correspondingly named capture group from the regex and parses it into the receiver type (needs type inference to work).
// 
// Limitations:
// This only works for somewhat "simple" expressions, e.g. array syntax, function calls, tuples, binary/unary operations, if expressions etc.
// More complex expressions, e.g. loops, match expressions, closures, would require semantic analysis / construction of a symbol table to get right.
// (Take e.g. a closure: `|a, b| a + b`. Here `a` and `b` are local bindings that don't have a counterpart in the regex. On the other hand, in the following
// closure: `|a| a + b`, only `a` is a local binding, `b` is captured from the environment and one could assume should be extracted from the regex.)
//
// Some expressions are also ambiguous:
// Take `foo.0`: Is `foo` a static variable? Or did we just parse it from the regex?
//
// Finally, macros don't work as of yet.
//
// Future extensions:
// * use_as_str attribute: FromStr is not implemented for &str. If a parameter is annotated with use_as_str, don't call parse(), but use &str directly
// * parse_as attribute: In case type inference fails
// * Semantic analysis: see above
// * Macros?

#[derive(Debug)]
pub struct TransformIdents {
    replaced_expression: Option<Expr>
}

impl TransformIdents {
    pub fn new() -> Self {
        Self {
            replaced_expression: None,
        }
    }
}

impl VisitMut for TransformIdents {
    fn visit_expr_array_mut(&mut self, expr_array: &mut ExprArray) {
        for mut el in Punctuated::pairs_mut(&mut expr_array.elems) {
            let it = el.value_mut();
            self.visit_expr_mut(it);
            if let Some(expr) = self.replaced_expression.take() {
                **it = expr;
                println!("Replaced expression in <ExprArray>");
            }
        }
    }

    fn visit_expr_call_mut(&mut self, expr_call: &mut ExprCall) {
        for mut el in Punctuated::pairs_mut(&mut expr_call.args) {
            let it = el.value_mut();
            self.visit_expr_mut(it);
            if let Some(expr) = self.replaced_expression.take() {
                **it = expr;
                println!("Replaced expression in <ExprCall>");
            }
        }
    }

    #[allow(clippy::single_match)]
    fn visit_expr_method_call_mut(&mut self, expr_method_call: &mut ExprMethodCall) {
        // Whitelist some exceptions, e.g. a range
        match *expr_method_call.receiver {
            Expr::Paren(ref mut paren) => match *paren.expr {
                Expr::Range(ref mut range) => self.visit_expr_range_mut(range),
                _ => {},
            }
            _ => {},
        }

        for mut el in Punctuated::pairs_mut(&mut expr_method_call.args) {
            let it = el.value_mut();
            self.visit_expr_mut(it);
            if let Some(expr) = self.replaced_expression.take() {
                **it = expr;
                println!("Replaced expression in <ExprMethodCall>");
            }
        }
    }

    fn visit_expr_tuple_mut(&mut self, expr_tuple: &mut ExprTuple) {
        for mut el in Punctuated::pairs_mut(&mut expr_tuple.elems) {
            let it = el.value_mut();
            self.visit_expr_mut(it);
            if let Some(expr) = self.replaced_expression.take() {
                **it = expr;
                println!("Replace expression in <ExprTuple>");
            }
        }
    }

    fn visit_expr_binary_mut(&mut self, expr_binary: &mut ExprBinary) {
        self.visit_expr_mut(&mut *expr_binary.left);
        if let Some(expr) = self.replaced_expression.take() {
            *expr_binary.left = expr;
            println!("Replace expression in <ExprBinary:Left>");
        }
        self.visit_expr_mut(&mut *expr_binary.right);
        if let Some(expr) = self.replaced_expression.take() {
            *expr_binary.right = expr;
            println!("Replace expression in <ExprBinary:Right>");
        }
    }

    fn visit_expr_unary_mut(&mut self, expr_unary: &mut ExprUnary) {
        self.visit_expr_mut(&mut *expr_unary.expr);
        if let Some(expr) = self.replaced_expression.take() {
            *expr_unary.expr = expr;
            println!("Replace expression in <ExprUnary>");
        }
    }

    fn visit_expr_cast_mut(&mut self, expr_cast: &mut ExprCast) {
        self.visit_expr_mut(&mut *expr_cast.expr);
        if let Some(expr) = self.replaced_expression.take() {
            *expr_cast.expr = expr;
            println!("Replace expression in <ExprCast>");
        }
    }

    fn visit_expr_if_mut(&mut self, expr_if: &mut ExprIf) {
        self.visit_expr_mut(&mut *expr_if.cond);
        if let Some(expr) = self.replaced_expression.take() {
            *expr_if.cond = expr;
            println!("Replace expression in <ExprIf:Cond>");
        }
        
        self.visit_block_mut(&mut expr_if.then_branch);
        if let Some(ref mut it) = expr_if.else_branch {
            self.visit_expr_mut(&mut *(it).1);
        };
    }

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        match *stmt {
            Stmt::Local(_) | Stmt::Item(_) | Stmt::Semi(_, _) => {
                panic!("Only expressions are allowed in construct_with attribute, this includes inside if/else blocks");
            }
            Stmt::Expr(ref mut expr) => {
                self.visit_expr_mut(expr);
                if let Some(new_expr) = self.replaced_expression.take() {
                    *expr = new_expr;
                    println!("Replace expression in <Stmt:Expr>");
                }
            }
        }
    }

    fn visit_expr_range_mut(&mut self, expr_range: &mut ExprRange) {
        if let Some(ref mut it) = expr_range.from {
            self.visit_expr_mut(&mut **it);
            if let Some(expr) = self.replaced_expression.take() {
                **it = expr;
                println!("Replace expression in <ExprRange:From>");
            }
        };
        if let Some(ref mut it) = expr_range.to {
            self.visit_expr_mut(&mut **it);
            if let Some(expr) = self.replaced_expression.take() {
                **it = expr;
                println!("Replace expression in <ExprRange:To>");
            }
        };
    }

    fn visit_expr_reference_mut(&mut self, expr_ref: &mut ExprReference) {
        self.visit_expr_mut(&mut *expr_ref.expr);
        if let Some(expr) = self.replaced_expression.take() {
            *expr_ref.expr = expr;
            println!("Replace expression in <ExprReference>");
        }
    }

    fn visit_expr_struct_mut(&mut self, expr_struct: &mut ExprStruct) {
        for mut el in Punctuated::pairs_mut(&mut expr_struct.fields) {
            let it = el.value_mut();
            self.visit_field_value_mut(it)
        }
        if let Some(ref mut it) = expr_struct.rest {
            self.visit_expr_mut(&mut **it)
        };
    }

    fn visit_field_value_mut(&mut self, field_value: &mut FieldValue) {
        self.visit_expr_mut(&mut field_value.expr);
        if let Some(expr) = self.replaced_expression.take() {
            field_value.expr = expr;

            // In case syntax without colon is used, we need to "desugar": Since the left side of the colon corresponds to
            // the struct member, we don't need to transform it. However, the right side refers to an identifier, which is assigned to the
            // struct member. This expression needs to be parsed from the regex. In order for this to work, we need to re-add the colon.
            // In essence, we're transforming `Struct { x }` to `Struct { x: parse_expr("x") }`.
            if field_value.colon_token.is_none() {
                field_value.colon_token = Some(syn::token::Colon { spans: [field_value.span()]});
            }
            println!("Replace expression in <FieldValue:Expr>");
        }
    }

    fn visit_expr_paren_mut(&mut self, expr_paren: &mut ExprParen) {
        self.visit_expr_mut(&mut *expr_paren.expr);
        if let Some(expr) = self.replaced_expression.take() {
            *expr_paren.expr = expr;
            println!("Replace expression in <ExprParen>");
        }
    }

    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        if ident != "None" {
            println!("Found identifier: {:?}", ident);

            let ident_as_string = ident.to_string();
            let ts = quote_spanned!(ident.span() => captures.name(#ident_as_string).unwrap().as_str().parse()?);
            let expr: Expr = parse2(ts).unwrap();
            self.replaced_expression = Some(expr);
        }
    }
}