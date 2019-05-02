#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
pub mod ast {
    use std::fmt;
    #[structural_match]
    pub struct Location(pub usize, pub usize);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Location {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Location(ref __self_0_0, ref __self_0_1) => {
                    let mut debug_trait_builder = f.debug_tuple("Location");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    let _ = debug_trait_builder.field(&&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Location {
        #[inline]
        fn clone(&self) -> Location {
            match *self {
                Location(ref __self_0_0, ref __self_0_1) => Location(
                    ::std::clone::Clone::clone(&(*__self_0_0)),
                    ::std::clone::Clone::clone(&(*__self_0_1)),
                ),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Location {
        #[inline]
        fn eq(&self, other: &Location) -> bool {
            match *other {
                Location(ref __self_1_0, ref __self_1_1) => match *self {
                    Location(ref __self_0_0, ref __self_0_1) => {
                        (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Location) -> bool {
            match *other {
                Location(ref __self_1_0, ref __self_1_1) => match *self {
                    Location(ref __self_0_0, ref __self_0_1) => {
                        (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Location {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<usize>;
                let _: ::std::cmp::AssertParamIsEq<usize>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for Location {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Location(ref __self_0_0, ref __self_0_1) => {
                    ::std::hash::Hash::hash(&(*__self_0_0), state);
                    ::std::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    #[structural_match]
    pub enum TokenType {
        This,
        Identifier,
        OpAdditive,
        OpMultiplicative,
        ShiftOperator,
        EqualityOperator,
        RelationalOperator,
        BitwiseOrOperator,
        BitwiseAndOperator,
        LogicalOrOperator,
        LogicalAndOperator,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for TokenType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&TokenType::This,) => {
                    let mut debug_trait_builder = f.debug_tuple("This");
                    debug_trait_builder.finish()
                }
                (&TokenType::Identifier,) => {
                    let mut debug_trait_builder = f.debug_tuple("Identifier");
                    debug_trait_builder.finish()
                }
                (&TokenType::OpAdditive,) => {
                    let mut debug_trait_builder = f.debug_tuple("OpAdditive");
                    debug_trait_builder.finish()
                }
                (&TokenType::OpMultiplicative,) => {
                    let mut debug_trait_builder = f.debug_tuple("OpMultiplicative");
                    debug_trait_builder.finish()
                }
                (&TokenType::ShiftOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("ShiftOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::EqualityOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("EqualityOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::RelationalOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("RelationalOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::BitwiseOrOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseOrOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::BitwiseAndOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseAndOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::LogicalOrOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("LogicalOrOperator");
                    debug_trait_builder.finish()
                }
                (&TokenType::LogicalAndOperator,) => {
                    let mut debug_trait_builder = f.debug_tuple("LogicalAndOperator");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for TokenType {
        #[inline]
        fn clone(&self) -> TokenType {
            match (&*self,) {
                (&TokenType::This,) => TokenType::This,
                (&TokenType::Identifier,) => TokenType::Identifier,
                (&TokenType::OpAdditive,) => TokenType::OpAdditive,
                (&TokenType::OpMultiplicative,) => TokenType::OpMultiplicative,
                (&TokenType::ShiftOperator,) => TokenType::ShiftOperator,
                (&TokenType::EqualityOperator,) => TokenType::EqualityOperator,
                (&TokenType::RelationalOperator,) => TokenType::RelationalOperator,
                (&TokenType::BitwiseOrOperator,) => TokenType::BitwiseOrOperator,
                (&TokenType::BitwiseAndOperator,) => TokenType::BitwiseAndOperator,
                (&TokenType::LogicalOrOperator,) => TokenType::LogicalOrOperator,
                (&TokenType::LogicalAndOperator,) => TokenType::LogicalAndOperator,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for TokenType {
        #[inline]
        fn eq(&self, other: &TokenType) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for TokenType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for TokenType {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                _ => ::std::hash::Hash::hash(
                    &unsafe { ::std::intrinsics::discriminant_value(self) },
                    state,
                ),
            }
        }
    }
    #[structural_match]
    pub struct Token {
        pub kind: TokenType,
        pub location: Location,
        pub value: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Token {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Token {
                    kind: ref __self_0_0,
                    location: ref __self_0_1,
                    value: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Token");
                    let _ = debug_trait_builder.field("kind", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("location", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("value", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Token {
        #[inline]
        fn eq(&self, other: &Token) -> bool {
            match *other {
                Token {
                    kind: ref __self_1_0,
                    location: ref __self_1_1,
                    value: ref __self_1_2,
                } => match *self {
                    Token {
                        kind: ref __self_0_0,
                        location: ref __self_0_1,
                        value: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Token) -> bool {
            match *other {
                Token {
                    kind: ref __self_1_0,
                    location: ref __self_1_1,
                    value: ref __self_1_2,
                } => match *self {
                    Token {
                        kind: ref __self_0_0,
                        location: ref __self_0_1,
                        value: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Token {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<TokenType>;
                let _: ::std::cmp::AssertParamIsEq<Location>;
                let _: ::std::cmp::AssertParamIsEq<String>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for Token {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Token {
                    kind: ref __self_0_0,
                    location: ref __self_0_1,
                    value: ref __self_0_2,
                } => {
                    ::std::hash::Hash::hash(&(*__self_0_0), state);
                    ::std::hash::Hash::hash(&(*__self_0_1), state);
                    ::std::hash::Hash::hash(&(*__self_0_2), state)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Token {
        #[inline]
        fn clone(&self) -> Token {
            match *self {
                Token {
                    kind: ref __self_0_0,
                    location: ref __self_0_1,
                    value: ref __self_0_2,
                } => Token {
                    kind: ::std::clone::Clone::clone(&(*__self_0_0)),
                    location: ::std::clone::Clone::clone(&(*__self_0_1)),
                    value: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    impl Token {
        pub fn new(location: Location, kind: TokenType, value: &str) -> Token {
            Token {
                location,
                value: value.to_owned(),
                kind,
            }
        }
    }
    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(
                &[""],
                &match (&self.value,) {
                    (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                },
            ))
        }
    }
    pub struct Object {
        entries: Vec<ObjectEntry>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Object {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Object {
                    entries: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Object");
                    let _ = debug_trait_builder.field("entries", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Object {
        #[inline]
        fn eq(&self, other: &Object) -> bool {
            match *other {
                Object {
                    entries: ref __self_1_0,
                } => match *self {
                    Object {
                        entries: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Object) -> bool {
            match *other {
                Object {
                    entries: ref __self_1_0,
                } => match *self {
                    Object {
                        entries: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Object {
        #[inline]
        fn clone(&self) -> Object {
            match *self {
                Object {
                    entries: ref __self_0_0,
                } => Object {
                    entries: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl Object {
        pub fn new(entries: Vec<ObjectEntry>) -> Object {
            Object { entries }
        }
    }
    pub struct ObjectEntry {
        pub key: Expr,
        pub value: Expr,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ObjectEntry {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ObjectEntry {
                    key: ref __self_0_0,
                    value: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ObjectEntry");
                    let _ = debug_trait_builder.field("key", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("value", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ObjectEntry {
        #[inline]
        fn eq(&self, other: &ObjectEntry) -> bool {
            match *other {
                ObjectEntry {
                    key: ref __self_1_0,
                    value: ref __self_1_1,
                } => match *self {
                    ObjectEntry {
                        key: ref __self_0_0,
                        value: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ObjectEntry) -> bool {
            match *other {
                ObjectEntry {
                    key: ref __self_1_0,
                    value: ref __self_1_1,
                } => match *self {
                    ObjectEntry {
                        key: ref __self_0_0,
                        value: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ObjectEntry {
        #[inline]
        fn clone(&self) -> ObjectEntry {
            match *self {
                ObjectEntry {
                    key: ref __self_0_0,
                    value: ref __self_0_1,
                } => ObjectEntry {
                    key: ::std::clone::Clone::clone(&(*__self_0_0)),
                    value: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl ObjectEntry {
        pub fn new(key: Expr, value: Expr) -> ObjectEntry {
            ObjectEntry { key, value }
        }
    }
    pub trait StmtVisitor<R> {
        fn visit_program_stmt(&mut self, e: &ProgramStmt) -> R;
        fn visit_var_stmt(&mut self, e: &VarStmt) -> R;
        fn visit_varlist_stmt(&mut self, e: &VarListStmt) -> R;
        fn visit_expr_stmt(&mut self, e: &ExprStmt) -> R;
        fn visit_func_stmt(&mut self, e: &FuncStmt) -> R;
        fn visit_class_stmt(&mut self, e: &ClassStmt) -> R;
        fn visit_interface_stmt(&mut self, e: &InterfaceStmt) -> R;
        fn visit_block_stmt(&mut self, e: &BlockStmt) -> R;
        fn visit_if_stmt(&mut self, e: &IfStmt) -> R;
        fn visit_for_stmt(&mut self, e: &ForStmt) -> R;
        fn visit_return_stmt(&mut self, e: &ReturnStmt) -> R;
        fn visit_continue_stmt(&mut self, e: &ContinueStmt) -> R;
        fn visit_break_stmt(&mut self, e: &BreakStmt) -> R;
    }
    pub trait ExprVisitor<R> {
        fn visit_assign_expr(&mut self, e: &AssignExpr) -> R;
        fn visit_call_expr(&mut self, e: &CallExpr) -> R;
        fn visit_literal_expr(&mut self, e: &LiteralExpr) -> R;
        fn visit_binary_expr(&mut self, e: &BinaryExpr) -> R;
        fn visit_member_expr(&mut self, e: &MemberExpr) -> R;
        fn visit_lookup_expr(&mut self, e: &LookupExpr) -> R;
        fn visit_arguments_expr(&mut self, e: &ArgumentsExpr) -> R;
        fn visit_logical_expr(&mut self, e: &LogicalExpr) -> R;
        fn visit_this_expr(&mut self, e: &ThisExpr) -> R;
        fn visit_var_expr(&mut self, e: &VarExpr) -> R;
        fn visit_identifier_expr(&mut self, e: &IdentifierExpr) -> R;
        fn visit_unary_expr(&mut self, e: &UnaryExpr) -> R;
        fn visit_postfix_expr(&mut self, e: &PostfixExpr) -> R;
    }
    pub enum UnaryOperator {
        Plus,
        Minus,
        Increment,
        Decrement,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UnaryOperator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&UnaryOperator::Plus,) => {
                    let mut debug_trait_builder = f.debug_tuple("Plus");
                    debug_trait_builder.finish()
                }
                (&UnaryOperator::Minus,) => {
                    let mut debug_trait_builder = f.debug_tuple("Minus");
                    debug_trait_builder.finish()
                }
                (&UnaryOperator::Increment,) => {
                    let mut debug_trait_builder = f.debug_tuple("Increment");
                    debug_trait_builder.finish()
                }
                (&UnaryOperator::Decrement,) => {
                    let mut debug_trait_builder = f.debug_tuple("Decrement");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UnaryOperator {
        #[inline]
        fn clone(&self) -> UnaryOperator {
            match (&*self,) {
                (&UnaryOperator::Plus,) => UnaryOperator::Plus,
                (&UnaryOperator::Minus,) => UnaryOperator::Minus,
                (&UnaryOperator::Increment,) => UnaryOperator::Increment,
                (&UnaryOperator::Decrement,) => UnaryOperator::Decrement,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UnaryOperator {
        #[inline]
        fn eq(&self, other: &UnaryOperator) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    pub enum PostfixOperator {
        Increment,
        Decrement,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for PostfixOperator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&PostfixOperator::Increment,) => {
                    let mut debug_trait_builder = f.debug_tuple("Increment");
                    debug_trait_builder.finish()
                }
                (&PostfixOperator::Decrement,) => {
                    let mut debug_trait_builder = f.debug_tuple("Decrement");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PostfixOperator {
        #[inline]
        fn clone(&self) -> PostfixOperator {
            match (&*self,) {
                (&PostfixOperator::Increment,) => PostfixOperator::Increment,
                (&PostfixOperator::Decrement,) => PostfixOperator::Decrement,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for PostfixOperator {
        #[inline]
        fn eq(&self, other: &PostfixOperator) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    pub enum BinaryOperator {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        BitwiseXor,
        BitwiseAnd,
        BitwiseOr,
        ShiftLeft,
        ShiftRight,
        Eq,
        Neq,
        Lt,
        Lte,
        Gt,
        Gte,
        Is,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BinaryOperator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&BinaryOperator::Add,) => {
                    let mut debug_trait_builder = f.debug_tuple("Add");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Sub,) => {
                    let mut debug_trait_builder = f.debug_tuple("Sub");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Mul,) => {
                    let mut debug_trait_builder = f.debug_tuple("Mul");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Div,) => {
                    let mut debug_trait_builder = f.debug_tuple("Div");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Mod,) => {
                    let mut debug_trait_builder = f.debug_tuple("Mod");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::BitwiseXor,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseXor");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::BitwiseAnd,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseAnd");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::BitwiseOr,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseOr");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::ShiftLeft,) => {
                    let mut debug_trait_builder = f.debug_tuple("ShiftLeft");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::ShiftRight,) => {
                    let mut debug_trait_builder = f.debug_tuple("ShiftRight");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Eq,) => {
                    let mut debug_trait_builder = f.debug_tuple("Eq");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Neq,) => {
                    let mut debug_trait_builder = f.debug_tuple("Neq");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Lt,) => {
                    let mut debug_trait_builder = f.debug_tuple("Lt");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Lte,) => {
                    let mut debug_trait_builder = f.debug_tuple("Lte");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Gt,) => {
                    let mut debug_trait_builder = f.debug_tuple("Gt");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Gte,) => {
                    let mut debug_trait_builder = f.debug_tuple("Gte");
                    debug_trait_builder.finish()
                }
                (&BinaryOperator::Is,) => {
                    let mut debug_trait_builder = f.debug_tuple("Is");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BinaryOperator {
        #[inline]
        fn clone(&self) -> BinaryOperator {
            match (&*self,) {
                (&BinaryOperator::Add,) => BinaryOperator::Add,
                (&BinaryOperator::Sub,) => BinaryOperator::Sub,
                (&BinaryOperator::Mul,) => BinaryOperator::Mul,
                (&BinaryOperator::Div,) => BinaryOperator::Div,
                (&BinaryOperator::Mod,) => BinaryOperator::Mod,
                (&BinaryOperator::BitwiseXor,) => BinaryOperator::BitwiseXor,
                (&BinaryOperator::BitwiseAnd,) => BinaryOperator::BitwiseAnd,
                (&BinaryOperator::BitwiseOr,) => BinaryOperator::BitwiseOr,
                (&BinaryOperator::ShiftLeft,) => BinaryOperator::ShiftLeft,
                (&BinaryOperator::ShiftRight,) => BinaryOperator::ShiftRight,
                (&BinaryOperator::Eq,) => BinaryOperator::Eq,
                (&BinaryOperator::Neq,) => BinaryOperator::Neq,
                (&BinaryOperator::Lt,) => BinaryOperator::Lt,
                (&BinaryOperator::Lte,) => BinaryOperator::Lte,
                (&BinaryOperator::Gt,) => BinaryOperator::Gt,
                (&BinaryOperator::Gte,) => BinaryOperator::Gte,
                (&BinaryOperator::Is,) => BinaryOperator::Is,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BinaryOperator {
        #[inline]
        fn eq(&self, other: &BinaryOperator) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    pub enum ComparisonOperator {
        Eq,
        Neq,
        Lt,
        Lte,
        Gt,
        Gte,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ComparisonOperator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ComparisonOperator::Eq,) => {
                    let mut debug_trait_builder = f.debug_tuple("Eq");
                    debug_trait_builder.finish()
                }
                (&ComparisonOperator::Neq,) => {
                    let mut debug_trait_builder = f.debug_tuple("Neq");
                    debug_trait_builder.finish()
                }
                (&ComparisonOperator::Lt,) => {
                    let mut debug_trait_builder = f.debug_tuple("Lt");
                    debug_trait_builder.finish()
                }
                (&ComparisonOperator::Lte,) => {
                    let mut debug_trait_builder = f.debug_tuple("Lte");
                    debug_trait_builder.finish()
                }
                (&ComparisonOperator::Gt,) => {
                    let mut debug_trait_builder = f.debug_tuple("Gt");
                    debug_trait_builder.finish()
                }
                (&ComparisonOperator::Gte,) => {
                    let mut debug_trait_builder = f.debug_tuple("Gte");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ComparisonOperator {
        #[inline]
        fn clone(&self) -> ComparisonOperator {
            match (&*self,) {
                (&ComparisonOperator::Eq,) => ComparisonOperator::Eq,
                (&ComparisonOperator::Neq,) => ComparisonOperator::Neq,
                (&ComparisonOperator::Lt,) => ComparisonOperator::Lt,
                (&ComparisonOperator::Lte,) => ComparisonOperator::Lte,
                (&ComparisonOperator::Gt,) => ComparisonOperator::Gt,
                (&ComparisonOperator::Gte,) => ComparisonOperator::Gte,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ComparisonOperator {
        #[inline]
        fn eq(&self, other: &ComparisonOperator) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    pub enum LogicalOperator {
        And,
        Or,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LogicalOperator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&LogicalOperator::And,) => {
                    let mut debug_trait_builder = f.debug_tuple("And");
                    debug_trait_builder.finish()
                }
                (&LogicalOperator::Or,) => {
                    let mut debug_trait_builder = f.debug_tuple("Or");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LogicalOperator {
        #[inline]
        fn clone(&self) -> LogicalOperator {
            match (&*self,) {
                (&LogicalOperator::And,) => LogicalOperator::And,
                (&LogicalOperator::Or,) => LogicalOperator::Or,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LogicalOperator {
        #[inline]
        fn eq(&self, other: &LogicalOperator) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    pub enum AssignmentOperator {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        ShiftLeft,
        ShiftRight,
        BitwiseAnd,
        BitwiseOr,
        BitwiseXor,
        Assign,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for AssignmentOperator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&AssignmentOperator::Add,) => {
                    let mut debug_trait_builder = f.debug_tuple("Add");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::Sub,) => {
                    let mut debug_trait_builder = f.debug_tuple("Sub");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::Mul,) => {
                    let mut debug_trait_builder = f.debug_tuple("Mul");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::Div,) => {
                    let mut debug_trait_builder = f.debug_tuple("Div");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::Mod,) => {
                    let mut debug_trait_builder = f.debug_tuple("Mod");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::ShiftLeft,) => {
                    let mut debug_trait_builder = f.debug_tuple("ShiftLeft");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::ShiftRight,) => {
                    let mut debug_trait_builder = f.debug_tuple("ShiftRight");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::BitwiseAnd,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseAnd");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::BitwiseOr,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseOr");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::BitwiseXor,) => {
                    let mut debug_trait_builder = f.debug_tuple("BitwiseXor");
                    debug_trait_builder.finish()
                }
                (&AssignmentOperator::Assign,) => {
                    let mut debug_trait_builder = f.debug_tuple("Assign");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for AssignmentOperator {
        #[inline]
        fn clone(&self) -> AssignmentOperator {
            match (&*self,) {
                (&AssignmentOperator::Add,) => AssignmentOperator::Add,
                (&AssignmentOperator::Sub,) => AssignmentOperator::Sub,
                (&AssignmentOperator::Mul,) => AssignmentOperator::Mul,
                (&AssignmentOperator::Div,) => AssignmentOperator::Div,
                (&AssignmentOperator::Mod,) => AssignmentOperator::Mod,
                (&AssignmentOperator::ShiftLeft,) => AssignmentOperator::ShiftLeft,
                (&AssignmentOperator::ShiftRight,) => AssignmentOperator::ShiftRight,
                (&AssignmentOperator::BitwiseAnd,) => AssignmentOperator::BitwiseAnd,
                (&AssignmentOperator::BitwiseOr,) => AssignmentOperator::BitwiseOr,
                (&AssignmentOperator::BitwiseXor,) => AssignmentOperator::BitwiseXor,
                (&AssignmentOperator::Assign,) => AssignmentOperator::Assign,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for AssignmentOperator {
        #[inline]
        fn eq(&self, other: &AssignmentOperator) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    pub enum Number {
        Double(f64),
        Integer(i64),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Number {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Number::Double(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Double");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Number::Integer(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Integer");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Number {
        #[inline]
        fn clone(&self) -> Number {
            match (&*self,) {
                (&Number::Double(ref __self_0),) => {
                    Number::Double(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Number::Integer(ref __self_0),) => {
                    Number::Integer(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Number {
        #[inline]
        fn eq(&self, other: &Number) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Number::Double(ref __self_0), &Number::Double(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Number::Integer(ref __self_0), &Number::Integer(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Number) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Number::Double(ref __self_0), &Number::Double(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Number::Integer(ref __self_0), &Number::Integer(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    pub enum Literal {
        String(String),
        Number(Number),
        Boolean(bool),
        Array(Vec<Expr>),
        Object(Object),
        Null,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Literal {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Literal::String(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("String");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Literal::Number(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Number");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Literal::Boolean(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Boolean");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Literal::Array(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Array");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Literal::Object(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Object");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Literal::Null,) => {
                    let mut debug_trait_builder = f.debug_tuple("Null");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Literal {
        #[inline]
        fn clone(&self) -> Literal {
            match (&*self,) {
                (&Literal::String(ref __self_0),) => {
                    Literal::String(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Literal::Number(ref __self_0),) => {
                    Literal::Number(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Literal::Boolean(ref __self_0),) => {
                    Literal::Boolean(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Literal::Array(ref __self_0),) => {
                    Literal::Array(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Literal::Object(ref __self_0),) => {
                    Literal::Object(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Literal::Null,) => Literal::Null,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Literal {
        #[inline]
        fn eq(&self, other: &Literal) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Literal::String(ref __self_0), &Literal::String(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Literal::Number(ref __self_0), &Literal::Number(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Literal::Boolean(ref __self_0), &Literal::Boolean(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Literal::Array(ref __self_0), &Literal::Array(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Literal::Object(ref __self_0), &Literal::Object(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Literal) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Literal::String(ref __self_0), &Literal::String(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Literal::Number(ref __self_0), &Literal::Number(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Literal::Boolean(ref __self_0), &Literal::Boolean(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Literal::Array(ref __self_0), &Literal::Array(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Literal::Object(ref __self_0), &Literal::Object(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => false,
                    }
                } else {
                    true
                }
            }
        }
    }
    pub enum Argument {
        Regular(String),
        Rest(String),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Argument {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Argument::Regular(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Regular");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Argument::Rest(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Rest");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Argument {
        #[inline]
        fn clone(&self) -> Argument {
            match (&*self,) {
                (&Argument::Regular(ref __self_0),) => {
                    Argument::Regular(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Argument::Rest(ref __self_0),) => {
                    Argument::Rest(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Argument {
        #[inline]
        fn eq(&self, other: &Argument) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Argument::Regular(ref __self_0), &Argument::Regular(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Argument::Rest(ref __self_0), &Argument::Rest(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Argument) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Argument::Regular(ref __self_0), &Argument::Regular(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Argument::Rest(ref __self_0), &Argument::Rest(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    pub enum Stmt {
        Program(ProgramStmt),
        Var(VarStmt),
        VarList(VarListStmt),
        Expr(ExprStmt),
        Func(FuncStmt),
        Class(ClassStmt),
        Interface(InterfaceStmt),
        Block(BlockStmt),
        If(IfStmt),
        For(ForStmt),
        Return(ReturnStmt),
        Continue(ContinueStmt),
        Break(BreakStmt),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Stmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Stmt::Program(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Program");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Var(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Var");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::VarList(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("VarList");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Expr(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Expr");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Func(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Func");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Class(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Class");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Interface(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Interface");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Block(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Block");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::If(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("If");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::For(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("For");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Return(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Return");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Continue(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Continue");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Stmt::Break(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Break");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Stmt {
        #[inline]
        fn clone(&self) -> Stmt {
            match (&*self,) {
                (&Stmt::Program(ref __self_0),) => {
                    Stmt::Program(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Var(ref __self_0),) => Stmt::Var(::std::clone::Clone::clone(&(*__self_0))),
                (&Stmt::VarList(ref __self_0),) => {
                    Stmt::VarList(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Expr(ref __self_0),) => {
                    Stmt::Expr(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Func(ref __self_0),) => {
                    Stmt::Func(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Class(ref __self_0),) => {
                    Stmt::Class(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Interface(ref __self_0),) => {
                    Stmt::Interface(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Block(ref __self_0),) => {
                    Stmt::Block(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::If(ref __self_0),) => Stmt::If(::std::clone::Clone::clone(&(*__self_0))),
                (&Stmt::For(ref __self_0),) => Stmt::For(::std::clone::Clone::clone(&(*__self_0))),
                (&Stmt::Return(ref __self_0),) => {
                    Stmt::Return(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Continue(ref __self_0),) => {
                    Stmt::Continue(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Stmt::Break(ref __self_0),) => {
                    Stmt::Break(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Stmt {
        #[inline]
        fn eq(&self, other: &Stmt) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Stmt::Program(ref __self_0), &Stmt::Program(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Var(ref __self_0), &Stmt::Var(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::VarList(ref __self_0), &Stmt::VarList(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Expr(ref __self_0), &Stmt::Expr(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Func(ref __self_0), &Stmt::Func(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Class(ref __self_0), &Stmt::Class(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Interface(ref __self_0), &Stmt::Interface(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Block(ref __self_0), &Stmt::Block(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::If(ref __self_0), &Stmt::If(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::For(ref __self_0), &Stmt::For(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Return(ref __self_0), &Stmt::Return(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Continue(ref __self_0), &Stmt::Continue(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Stmt::Break(ref __self_0), &Stmt::Break(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Stmt) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Stmt::Program(ref __self_0), &Stmt::Program(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Var(ref __self_0), &Stmt::Var(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::VarList(ref __self_0), &Stmt::VarList(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Expr(ref __self_0), &Stmt::Expr(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Func(ref __self_0), &Stmt::Func(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Class(ref __self_0), &Stmt::Class(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Interface(ref __self_0), &Stmt::Interface(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Block(ref __self_0), &Stmt::Block(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::If(ref __self_0), &Stmt::If(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::For(ref __self_0), &Stmt::For(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Return(ref __self_0), &Stmt::Return(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Continue(ref __self_0), &Stmt::Continue(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Stmt::Break(ref __self_0), &Stmt::Break(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    impl Stmt {
        pub fn accept<R>(&self, visitor: &mut StmtVisitor<R>) -> R {
            match self {
                Stmt::Program(s) => visitor.visit_program_stmt(&s),
                Stmt::Var(s) => visitor.visit_var_stmt(&s),
                Stmt::VarList(s) => visitor.visit_varlist_stmt(&s),
                Stmt::Expr(s) => visitor.visit_expr_stmt(&s),
                Stmt::Func(s) => visitor.visit_func_stmt(&s),
                Stmt::Class(s) => visitor.visit_class_stmt(&s),
                Stmt::Interface(s) => visitor.visit_interface_stmt(&s),
                Stmt::Block(s) => visitor.visit_block_stmt(&s),
                Stmt::If(s) => visitor.visit_if_stmt(&s),
                Stmt::For(s) => visitor.visit_for_stmt(&s),
                Stmt::Return(s) => visitor.visit_return_stmt(&s),
                Stmt::Continue(s) => visitor.visit_continue_stmt(&s),
                Stmt::Break(s) => visitor.visit_break_stmt(&s),
            }
        }
    }
    pub enum Expr {
        Assign(AssignExpr),
        Call(CallExpr),
        Literal(LiteralExpr),
        Binary(BinaryExpr),
        Member(MemberExpr),
        Lookup(LookupExpr),
        Arguments(ArgumentsExpr),
        Logical(LogicalExpr),
        This(ThisExpr),
        Var(VarExpr),
        Identifier(IdentifierExpr),
        Unary(UnaryExpr),
        Postfix(PostfixExpr),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Expr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Expr::Assign(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Assign");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Call(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Call");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Literal(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Literal");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Binary(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Binary");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Member(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Member");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Lookup(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Lookup");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Arguments(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Arguments");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Logical(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Logical");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::This(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("This");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Var(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Var");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Identifier(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Identifier");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Unary(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Unary");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Expr::Postfix(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Postfix");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Expr {
        #[inline]
        fn clone(&self) -> Expr {
            match (&*self,) {
                (&Expr::Assign(ref __self_0),) => {
                    Expr::Assign(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Call(ref __self_0),) => {
                    Expr::Call(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Literal(ref __self_0),) => {
                    Expr::Literal(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Binary(ref __self_0),) => {
                    Expr::Binary(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Member(ref __self_0),) => {
                    Expr::Member(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Lookup(ref __self_0),) => {
                    Expr::Lookup(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Arguments(ref __self_0),) => {
                    Expr::Arguments(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Logical(ref __self_0),) => {
                    Expr::Logical(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::This(ref __self_0),) => {
                    Expr::This(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Var(ref __self_0),) => Expr::Var(::std::clone::Clone::clone(&(*__self_0))),
                (&Expr::Identifier(ref __self_0),) => {
                    Expr::Identifier(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Unary(ref __self_0),) => {
                    Expr::Unary(::std::clone::Clone::clone(&(*__self_0)))
                }
                (&Expr::Postfix(ref __self_0),) => {
                    Expr::Postfix(::std::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Expr {
        #[inline]
        fn eq(&self, other: &Expr) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Expr::Assign(ref __self_0), &Expr::Assign(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Call(ref __self_0), &Expr::Call(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Literal(ref __self_0), &Expr::Literal(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Binary(ref __self_0), &Expr::Binary(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Member(ref __self_0), &Expr::Member(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Lookup(ref __self_0), &Expr::Lookup(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Arguments(ref __self_0), &Expr::Arguments(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Logical(ref __self_0), &Expr::Logical(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::This(ref __self_0), &Expr::This(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Var(ref __self_0), &Expr::Var(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Identifier(ref __self_0), &Expr::Identifier(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Unary(ref __self_0), &Expr::Unary(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Expr::Postfix(ref __self_0), &Expr::Postfix(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Expr) -> bool {
            {
                let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Expr::Assign(ref __self_0), &Expr::Assign(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Call(ref __self_0), &Expr::Call(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Literal(ref __self_0), &Expr::Literal(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Binary(ref __self_0), &Expr::Binary(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Member(ref __self_0), &Expr::Member(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Lookup(ref __self_0), &Expr::Lookup(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Arguments(ref __self_0), &Expr::Arguments(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Logical(ref __self_0), &Expr::Logical(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::This(ref __self_0), &Expr::This(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Var(ref __self_0), &Expr::Var(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Identifier(ref __self_0), &Expr::Identifier(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Unary(ref __self_0), &Expr::Unary(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Expr::Postfix(ref __self_0), &Expr::Postfix(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::std::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    impl Expr {
        pub fn accept<R>(&self, visitor: &mut ExprVisitor<R>) -> R {
            match self {
                Expr::Assign(s) => visitor.visit_assign_expr(&s),
                Expr::Call(s) => visitor.visit_call_expr(&s),
                Expr::Literal(s) => visitor.visit_literal_expr(&s),
                Expr::Binary(s) => visitor.visit_binary_expr(&s),
                Expr::Member(s) => visitor.visit_member_expr(&s),
                Expr::Lookup(s) => visitor.visit_lookup_expr(&s),
                Expr::Arguments(s) => visitor.visit_arguments_expr(&s),
                Expr::Logical(s) => visitor.visit_logical_expr(&s),
                Expr::This(s) => visitor.visit_this_expr(&s),
                Expr::Var(s) => visitor.visit_var_expr(&s),
                Expr::Identifier(s) => visitor.visit_identifier_expr(&s),
                Expr::Unary(s) => visitor.visit_unary_expr(&s),
                Expr::Postfix(s) => visitor.visit_postfix_expr(&s),
            }
        }
    }
    pub struct ProgramStmt {
        pub statements: Vec<Box<Stmt>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ProgramStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ProgramStmt {
                    statements: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ProgramStmt");
                    let _ = debug_trait_builder.field("statements", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ProgramStmt {
        #[inline]
        fn clone(&self) -> ProgramStmt {
            match *self {
                ProgramStmt {
                    statements: ref __self_0_0,
                } => ProgramStmt {
                    statements: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ProgramStmt {
        #[inline]
        fn eq(&self, other: &ProgramStmt) -> bool {
            match *other {
                ProgramStmt {
                    statements: ref __self_1_0,
                } => match *self {
                    ProgramStmt {
                        statements: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ProgramStmt) -> bool {
            match *other {
                ProgramStmt {
                    statements: ref __self_1_0,
                } => match *self {
                    ProgramStmt {
                        statements: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ProgramStmt {
        pub fn new(statements: Vec<Box<Stmt>>) -> ProgramStmt {
            ProgramStmt { statements }
        }
    }
    pub struct VarStmt {
        pub name: String,
        pub initializer: Option<Expr>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for VarStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                VarStmt {
                    name: ref __self_0_0,
                    initializer: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("VarStmt");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("initializer", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for VarStmt {
        #[inline]
        fn clone(&self) -> VarStmt {
            match *self {
                VarStmt {
                    name: ref __self_0_0,
                    initializer: ref __self_0_1,
                } => VarStmt {
                    name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    initializer: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for VarStmt {
        #[inline]
        fn eq(&self, other: &VarStmt) -> bool {
            match *other {
                VarStmt {
                    name: ref __self_1_0,
                    initializer: ref __self_1_1,
                } => match *self {
                    VarStmt {
                        name: ref __self_0_0,
                        initializer: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VarStmt) -> bool {
            match *other {
                VarStmt {
                    name: ref __self_1_0,
                    initializer: ref __self_1_1,
                } => match *self {
                    VarStmt {
                        name: ref __self_0_0,
                        initializer: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl VarStmt {
        pub fn new(name: String, initializer: Option<Expr>) -> VarStmt {
            VarStmt { name, initializer }
        }
    }
    pub struct VarListStmt {
        pub variables: Vec<VarStmt>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for VarListStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                VarListStmt {
                    variables: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("VarListStmt");
                    let _ = debug_trait_builder.field("variables", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for VarListStmt {
        #[inline]
        fn clone(&self) -> VarListStmt {
            match *self {
                VarListStmt {
                    variables: ref __self_0_0,
                } => VarListStmt {
                    variables: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for VarListStmt {
        #[inline]
        fn eq(&self, other: &VarListStmt) -> bool {
            match *other {
                VarListStmt {
                    variables: ref __self_1_0,
                } => match *self {
                    VarListStmt {
                        variables: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VarListStmt) -> bool {
            match *other {
                VarListStmt {
                    variables: ref __self_1_0,
                } => match *self {
                    VarListStmt {
                        variables: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl VarListStmt {
        pub fn new(variables: Vec<VarStmt>) -> VarListStmt {
            VarListStmt { variables }
        }
    }
    pub struct ExprStmt {
        pub expression: Expr,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ExprStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ExprStmt {
                    expression: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ExprStmt");
                    let _ = debug_trait_builder.field("expression", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ExprStmt {
        #[inline]
        fn clone(&self) -> ExprStmt {
            match *self {
                ExprStmt {
                    expression: ref __self_0_0,
                } => ExprStmt {
                    expression: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ExprStmt {
        #[inline]
        fn eq(&self, other: &ExprStmt) -> bool {
            match *other {
                ExprStmt {
                    expression: ref __self_1_0,
                } => match *self {
                    ExprStmt {
                        expression: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ExprStmt) -> bool {
            match *other {
                ExprStmt {
                    expression: ref __self_1_0,
                } => match *self {
                    ExprStmt {
                        expression: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ExprStmt {
        pub fn new(expression: Expr) -> ExprStmt {
            ExprStmt { expression }
        }
    }
    pub struct FuncStmt {
        pub name: String,
        pub body: Box<Stmt>,
        pub parameters: Vec<Argument>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for FuncStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                FuncStmt {
                    name: ref __self_0_0,
                    body: ref __self_0_1,
                    parameters: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("FuncStmt");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("body", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("parameters", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for FuncStmt {
        #[inline]
        fn clone(&self) -> FuncStmt {
            match *self {
                FuncStmt {
                    name: ref __self_0_0,
                    body: ref __self_0_1,
                    parameters: ref __self_0_2,
                } => FuncStmt {
                    name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    body: ::std::clone::Clone::clone(&(*__self_0_1)),
                    parameters: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for FuncStmt {
        #[inline]
        fn eq(&self, other: &FuncStmt) -> bool {
            match *other {
                FuncStmt {
                    name: ref __self_1_0,
                    body: ref __self_1_1,
                    parameters: ref __self_1_2,
                } => match *self {
                    FuncStmt {
                        name: ref __self_0_0,
                        body: ref __self_0_1,
                        parameters: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &FuncStmt) -> bool {
            match *other {
                FuncStmt {
                    name: ref __self_1_0,
                    body: ref __self_1_1,
                    parameters: ref __self_1_2,
                } => match *self {
                    FuncStmt {
                        name: ref __self_0_0,
                        body: ref __self_0_1,
                        parameters: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl FuncStmt {
        pub fn new(name: String, body: Box<Stmt>, parameters: Vec<Argument>) -> FuncStmt {
            FuncStmt {
                name,
                body,
                parameters,
            }
        }
    }
    pub struct ClassStmt {
        pub name: String,
        pub members: Vec<Box<Stmt>>,
        pub extends: Option<String>,
        pub implements: Vec<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ClassStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ClassStmt {
                    name: ref __self_0_0,
                    members: ref __self_0_1,
                    extends: ref __self_0_2,
                    implements: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ClassStmt");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("members", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("extends", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("implements", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ClassStmt {
        #[inline]
        fn clone(&self) -> ClassStmt {
            match *self {
                ClassStmt {
                    name: ref __self_0_0,
                    members: ref __self_0_1,
                    extends: ref __self_0_2,
                    implements: ref __self_0_3,
                } => ClassStmt {
                    name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    members: ::std::clone::Clone::clone(&(*__self_0_1)),
                    extends: ::std::clone::Clone::clone(&(*__self_0_2)),
                    implements: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ClassStmt {
        #[inline]
        fn eq(&self, other: &ClassStmt) -> bool {
            match *other {
                ClassStmt {
                    name: ref __self_1_0,
                    members: ref __self_1_1,
                    extends: ref __self_1_2,
                    implements: ref __self_1_3,
                } => match *self {
                    ClassStmt {
                        name: ref __self_0_0,
                        members: ref __self_0_1,
                        extends: ref __self_0_2,
                        implements: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ClassStmt) -> bool {
            match *other {
                ClassStmt {
                    name: ref __self_1_0,
                    members: ref __self_1_1,
                    extends: ref __self_1_2,
                    implements: ref __self_1_3,
                } => match *self {
                    ClassStmt {
                        name: ref __self_0_0,
                        members: ref __self_0_1,
                        extends: ref __self_0_2,
                        implements: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    impl ClassStmt {
        pub fn new(
            name: String,
            members: Vec<Box<Stmt>>,
            extends: Option<String>,
            implements: Vec<String>,
        ) -> ClassStmt {
            ClassStmt {
                name,
                members,
                extends,
                implements,
            }
        }
    }
    pub struct InterfaceStmt {
        pub name: String,
        pub extends: Option<String>,
        pub members: Vec<Box<Stmt>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for InterfaceStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                InterfaceStmt {
                    name: ref __self_0_0,
                    extends: ref __self_0_1,
                    members: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("InterfaceStmt");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("extends", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("members", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for InterfaceStmt {
        #[inline]
        fn clone(&self) -> InterfaceStmt {
            match *self {
                InterfaceStmt {
                    name: ref __self_0_0,
                    extends: ref __self_0_1,
                    members: ref __self_0_2,
                } => InterfaceStmt {
                    name: ::std::clone::Clone::clone(&(*__self_0_0)),
                    extends: ::std::clone::Clone::clone(&(*__self_0_1)),
                    members: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for InterfaceStmt {
        #[inline]
        fn eq(&self, other: &InterfaceStmt) -> bool {
            match *other {
                InterfaceStmt {
                    name: ref __self_1_0,
                    extends: ref __self_1_1,
                    members: ref __self_1_2,
                } => match *self {
                    InterfaceStmt {
                        name: ref __self_0_0,
                        extends: ref __self_0_1,
                        members: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &InterfaceStmt) -> bool {
            match *other {
                InterfaceStmt {
                    name: ref __self_1_0,
                    extends: ref __self_1_1,
                    members: ref __self_1_2,
                } => match *self {
                    InterfaceStmt {
                        name: ref __self_0_0,
                        extends: ref __self_0_1,
                        members: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl InterfaceStmt {
        pub fn new(
            name: String,
            extends: Option<String>,
            members: Vec<Box<Stmt>>,
        ) -> InterfaceStmt {
            InterfaceStmt {
                name,
                extends,
                members,
            }
        }
    }
    pub struct BlockStmt {
        pub statements: Vec<Box<Stmt>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                BlockStmt {
                    statements: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("BlockStmt");
                    let _ = debug_trait_builder.field("statements", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BlockStmt {
        #[inline]
        fn clone(&self) -> BlockStmt {
            match *self {
                BlockStmt {
                    statements: ref __self_0_0,
                } => BlockStmt {
                    statements: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockStmt {
        #[inline]
        fn eq(&self, other: &BlockStmt) -> bool {
            match *other {
                BlockStmt {
                    statements: ref __self_1_0,
                } => match *self {
                    BlockStmt {
                        statements: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &BlockStmt) -> bool {
            match *other {
                BlockStmt {
                    statements: ref __self_1_0,
                } => match *self {
                    BlockStmt {
                        statements: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl BlockStmt {
        pub fn new(statements: Vec<Box<Stmt>>) -> BlockStmt {
            BlockStmt { statements }
        }
    }
    pub struct IfStmt {
        pub test: Expr,
        pub consequent: Box<Stmt>,
        pub alternative: Option<Box<Stmt>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for IfStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                IfStmt {
                    test: ref __self_0_0,
                    consequent: ref __self_0_1,
                    alternative: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("IfStmt");
                    let _ = debug_trait_builder.field("test", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("consequent", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("alternative", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for IfStmt {
        #[inline]
        fn clone(&self) -> IfStmt {
            match *self {
                IfStmt {
                    test: ref __self_0_0,
                    consequent: ref __self_0_1,
                    alternative: ref __self_0_2,
                } => IfStmt {
                    test: ::std::clone::Clone::clone(&(*__self_0_0)),
                    consequent: ::std::clone::Clone::clone(&(*__self_0_1)),
                    alternative: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for IfStmt {
        #[inline]
        fn eq(&self, other: &IfStmt) -> bool {
            match *other {
                IfStmt {
                    test: ref __self_1_0,
                    consequent: ref __self_1_1,
                    alternative: ref __self_1_2,
                } => match *self {
                    IfStmt {
                        test: ref __self_0_0,
                        consequent: ref __self_0_1,
                        alternative: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &IfStmt) -> bool {
            match *other {
                IfStmt {
                    test: ref __self_1_0,
                    consequent: ref __self_1_1,
                    alternative: ref __self_1_2,
                } => match *self {
                    IfStmt {
                        test: ref __self_0_0,
                        consequent: ref __self_0_1,
                        alternative: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl IfStmt {
        pub fn new(test: Expr, consequent: Box<Stmt>, alternative: Option<Box<Stmt>>) -> IfStmt {
            IfStmt {
                test,
                consequent,
                alternative,
            }
        }
    }
    pub struct ForStmt {
        pub element: Token,
        pub index: Option<Token>,
        pub iterator: Expr,
        pub body: Box<Stmt>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ForStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ForStmt {
                    element: ref __self_0_0,
                    index: ref __self_0_1,
                    iterator: ref __self_0_2,
                    body: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ForStmt");
                    let _ = debug_trait_builder.field("element", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("index", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("iterator", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("body", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ForStmt {
        #[inline]
        fn clone(&self) -> ForStmt {
            match *self {
                ForStmt {
                    element: ref __self_0_0,
                    index: ref __self_0_1,
                    iterator: ref __self_0_2,
                    body: ref __self_0_3,
                } => ForStmt {
                    element: ::std::clone::Clone::clone(&(*__self_0_0)),
                    index: ::std::clone::Clone::clone(&(*__self_0_1)),
                    iterator: ::std::clone::Clone::clone(&(*__self_0_2)),
                    body: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ForStmt {
        #[inline]
        fn eq(&self, other: &ForStmt) -> bool {
            match *other {
                ForStmt {
                    element: ref __self_1_0,
                    index: ref __self_1_1,
                    iterator: ref __self_1_2,
                    body: ref __self_1_3,
                } => match *self {
                    ForStmt {
                        element: ref __self_0_0,
                        index: ref __self_0_1,
                        iterator: ref __self_0_2,
                        body: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ForStmt) -> bool {
            match *other {
                ForStmt {
                    element: ref __self_1_0,
                    index: ref __self_1_1,
                    iterator: ref __self_1_2,
                    body: ref __self_1_3,
                } => match *self {
                    ForStmt {
                        element: ref __self_0_0,
                        index: ref __self_0_1,
                        iterator: ref __self_0_2,
                        body: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    impl ForStmt {
        pub fn new(
            element: Token,
            index: Option<Token>,
            iterator: Expr,
            body: Box<Stmt>,
        ) -> ForStmt {
            ForStmt {
                element,
                index,
                iterator,
                body,
            }
        }
    }
    pub struct ReturnStmt {
        pub expression: Option<Expr>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ReturnStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ReturnStmt {
                    expression: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ReturnStmt");
                    let _ = debug_trait_builder.field("expression", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ReturnStmt {
        #[inline]
        fn clone(&self) -> ReturnStmt {
            match *self {
                ReturnStmt {
                    expression: ref __self_0_0,
                } => ReturnStmt {
                    expression: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ReturnStmt {
        #[inline]
        fn eq(&self, other: &ReturnStmt) -> bool {
            match *other {
                ReturnStmt {
                    expression: ref __self_1_0,
                } => match *self {
                    ReturnStmt {
                        expression: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ReturnStmt) -> bool {
            match *other {
                ReturnStmt {
                    expression: ref __self_1_0,
                } => match *self {
                    ReturnStmt {
                        expression: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ReturnStmt {
        pub fn new(expression: Option<Expr>) -> ReturnStmt {
            ReturnStmt { expression }
        }
    }
    pub struct ContinueStmt {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ContinueStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ContinueStmt {} => {
                    let mut debug_trait_builder = f.debug_struct("ContinueStmt");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ContinueStmt {
        #[inline]
        fn clone(&self) -> ContinueStmt {
            match *self {
                ContinueStmt {} => ContinueStmt {},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ContinueStmt {
        #[inline]
        fn eq(&self, other: &ContinueStmt) -> bool {
            match *other {
                ContinueStmt {} => match *self {
                    ContinueStmt {} => true,
                },
            }
        }
    }
    impl ContinueStmt {
        pub fn new() -> ContinueStmt {
            ContinueStmt {}
        }
    }
    pub struct BreakStmt {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BreakStmt {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                BreakStmt {} => {
                    let mut debug_trait_builder = f.debug_struct("BreakStmt");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BreakStmt {
        #[inline]
        fn clone(&self) -> BreakStmt {
            match *self {
                BreakStmt {} => BreakStmt {},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BreakStmt {
        #[inline]
        fn eq(&self, other: &BreakStmt) -> bool {
            match *other {
                BreakStmt {} => match *self {
                    BreakStmt {} => true,
                },
            }
        }
    }
    impl BreakStmt {
        pub fn new() -> BreakStmt {
            BreakStmt {}
        }
    }
    pub struct AssignExpr {
        pub destination: Box<Expr>,
        pub value: Box<Expr>,
        pub operator: AssignmentOperator,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for AssignExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                AssignExpr {
                    destination: ref __self_0_0,
                    value: ref __self_0_1,
                    operator: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("AssignExpr");
                    let _ = debug_trait_builder.field("destination", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("value", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("operator", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for AssignExpr {
        #[inline]
        fn clone(&self) -> AssignExpr {
            match *self {
                AssignExpr {
                    destination: ref __self_0_0,
                    value: ref __self_0_1,
                    operator: ref __self_0_2,
                } => AssignExpr {
                    destination: ::std::clone::Clone::clone(&(*__self_0_0)),
                    value: ::std::clone::Clone::clone(&(*__self_0_1)),
                    operator: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for AssignExpr {
        #[inline]
        fn eq(&self, other: &AssignExpr) -> bool {
            match *other {
                AssignExpr {
                    destination: ref __self_1_0,
                    value: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    AssignExpr {
                        destination: ref __self_0_0,
                        value: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &AssignExpr) -> bool {
            match *other {
                AssignExpr {
                    destination: ref __self_1_0,
                    value: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    AssignExpr {
                        destination: ref __self_0_0,
                        value: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl AssignExpr {
        pub fn new(
            destination: Box<Expr>,
            value: Box<Expr>,
            operator: AssignmentOperator,
        ) -> AssignExpr {
            AssignExpr {
                destination,
                value,
                operator,
            }
        }
    }
    pub struct CallExpr {
        pub member: Box<Expr>,
        pub arguments: Vec<Expr>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for CallExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                CallExpr {
                    member: ref __self_0_0,
                    arguments: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("CallExpr");
                    let _ = debug_trait_builder.field("member", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("arguments", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for CallExpr {
        #[inline]
        fn clone(&self) -> CallExpr {
            match *self {
                CallExpr {
                    member: ref __self_0_0,
                    arguments: ref __self_0_1,
                } => CallExpr {
                    member: ::std::clone::Clone::clone(&(*__self_0_0)),
                    arguments: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for CallExpr {
        #[inline]
        fn eq(&self, other: &CallExpr) -> bool {
            match *other {
                CallExpr {
                    member: ref __self_1_0,
                    arguments: ref __self_1_1,
                } => match *self {
                    CallExpr {
                        member: ref __self_0_0,
                        arguments: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &CallExpr) -> bool {
            match *other {
                CallExpr {
                    member: ref __self_1_0,
                    arguments: ref __self_1_1,
                } => match *self {
                    CallExpr {
                        member: ref __self_0_0,
                        arguments: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl CallExpr {
        pub fn new(member: Box<Expr>, arguments: Vec<Expr>) -> CallExpr {
            CallExpr { member, arguments }
        }
    }
    pub struct LiteralExpr {
        pub value: Literal,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LiteralExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LiteralExpr {
                    value: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("LiteralExpr");
                    let _ = debug_trait_builder.field("value", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LiteralExpr {
        #[inline]
        fn clone(&self) -> LiteralExpr {
            match *self {
                LiteralExpr {
                    value: ref __self_0_0,
                } => LiteralExpr {
                    value: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LiteralExpr {
        #[inline]
        fn eq(&self, other: &LiteralExpr) -> bool {
            match *other {
                LiteralExpr {
                    value: ref __self_1_0,
                } => match *self {
                    LiteralExpr {
                        value: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LiteralExpr) -> bool {
            match *other {
                LiteralExpr {
                    value: ref __self_1_0,
                } => match *self {
                    LiteralExpr {
                        value: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl LiteralExpr {
        pub fn new(value: Literal) -> LiteralExpr {
            LiteralExpr { value }
        }
    }
    pub struct BinaryExpr {
        pub left: Box<Expr>,
        pub right: Box<Expr>,
        pub operator: BinaryOperator,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BinaryExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                BinaryExpr {
                    left: ref __self_0_0,
                    right: ref __self_0_1,
                    operator: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("BinaryExpr");
                    let _ = debug_trait_builder.field("left", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("right", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("operator", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BinaryExpr {
        #[inline]
        fn clone(&self) -> BinaryExpr {
            match *self {
                BinaryExpr {
                    left: ref __self_0_0,
                    right: ref __self_0_1,
                    operator: ref __self_0_2,
                } => BinaryExpr {
                    left: ::std::clone::Clone::clone(&(*__self_0_0)),
                    right: ::std::clone::Clone::clone(&(*__self_0_1)),
                    operator: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BinaryExpr {
        #[inline]
        fn eq(&self, other: &BinaryExpr) -> bool {
            match *other {
                BinaryExpr {
                    left: ref __self_1_0,
                    right: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    BinaryExpr {
                        left: ref __self_0_0,
                        right: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &BinaryExpr) -> bool {
            match *other {
                BinaryExpr {
                    left: ref __self_1_0,
                    right: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    BinaryExpr {
                        left: ref __self_0_0,
                        right: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl BinaryExpr {
        pub fn new(left: Box<Expr>, right: Box<Expr>, operator: BinaryOperator) -> BinaryExpr {
            BinaryExpr {
                left,
                right,
                operator,
            }
        }
    }
    pub struct MemberExpr {
        pub object: Box<Expr>,
        pub property: Box<Expr>,
        pub computed: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for MemberExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                MemberExpr {
                    object: ref __self_0_0,
                    property: ref __self_0_1,
                    computed: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("MemberExpr");
                    let _ = debug_trait_builder.field("object", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("property", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("computed", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for MemberExpr {
        #[inline]
        fn clone(&self) -> MemberExpr {
            match *self {
                MemberExpr {
                    object: ref __self_0_0,
                    property: ref __self_0_1,
                    computed: ref __self_0_2,
                } => MemberExpr {
                    object: ::std::clone::Clone::clone(&(*__self_0_0)),
                    property: ::std::clone::Clone::clone(&(*__self_0_1)),
                    computed: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for MemberExpr {
        #[inline]
        fn eq(&self, other: &MemberExpr) -> bool {
            match *other {
                MemberExpr {
                    object: ref __self_1_0,
                    property: ref __self_1_1,
                    computed: ref __self_1_2,
                } => match *self {
                    MemberExpr {
                        object: ref __self_0_0,
                        property: ref __self_0_1,
                        computed: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &MemberExpr) -> bool {
            match *other {
                MemberExpr {
                    object: ref __self_1_0,
                    property: ref __self_1_1,
                    computed: ref __self_1_2,
                } => match *self {
                    MemberExpr {
                        object: ref __self_0_0,
                        property: ref __self_0_1,
                        computed: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl MemberExpr {
        pub fn new(object: Box<Expr>, property: Box<Expr>, computed: bool) -> MemberExpr {
            MemberExpr {
                object,
                property,
                computed,
            }
        }
    }
    pub struct LookupExpr {
        pub token: Token,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LookupExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LookupExpr {
                    token: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("LookupExpr");
                    let _ = debug_trait_builder.field("token", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LookupExpr {
        #[inline]
        fn clone(&self) -> LookupExpr {
            match *self {
                LookupExpr {
                    token: ref __self_0_0,
                } => LookupExpr {
                    token: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LookupExpr {
        #[inline]
        fn eq(&self, other: &LookupExpr) -> bool {
            match *other {
                LookupExpr {
                    token: ref __self_1_0,
                } => match *self {
                    LookupExpr {
                        token: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LookupExpr) -> bool {
            match *other {
                LookupExpr {
                    token: ref __self_1_0,
                } => match *self {
                    LookupExpr {
                        token: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl LookupExpr {
        pub fn new(token: Token) -> LookupExpr {
            LookupExpr { token }
        }
    }
    pub struct ArgumentsExpr {
        pub expressions: Vec<Box<Expr>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ArgumentsExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ArgumentsExpr {
                    expressions: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ArgumentsExpr");
                    let _ = debug_trait_builder.field("expressions", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ArgumentsExpr {
        #[inline]
        fn clone(&self) -> ArgumentsExpr {
            match *self {
                ArgumentsExpr {
                    expressions: ref __self_0_0,
                } => ArgumentsExpr {
                    expressions: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ArgumentsExpr {
        #[inline]
        fn eq(&self, other: &ArgumentsExpr) -> bool {
            match *other {
                ArgumentsExpr {
                    expressions: ref __self_1_0,
                } => match *self {
                    ArgumentsExpr {
                        expressions: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ArgumentsExpr) -> bool {
            match *other {
                ArgumentsExpr {
                    expressions: ref __self_1_0,
                } => match *self {
                    ArgumentsExpr {
                        expressions: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ArgumentsExpr {
        pub fn new(expressions: Vec<Box<Expr>>) -> ArgumentsExpr {
            ArgumentsExpr { expressions }
        }
    }
    pub struct LogicalExpr {
        pub left: Box<Expr>,
        pub right: Box<Expr>,
        pub operator: LogicalOperator,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LogicalExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LogicalExpr {
                    left: ref __self_0_0,
                    right: ref __self_0_1,
                    operator: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("LogicalExpr");
                    let _ = debug_trait_builder.field("left", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("right", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("operator", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LogicalExpr {
        #[inline]
        fn clone(&self) -> LogicalExpr {
            match *self {
                LogicalExpr {
                    left: ref __self_0_0,
                    right: ref __self_0_1,
                    operator: ref __self_0_2,
                } => LogicalExpr {
                    left: ::std::clone::Clone::clone(&(*__self_0_0)),
                    right: ::std::clone::Clone::clone(&(*__self_0_1)),
                    operator: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LogicalExpr {
        #[inline]
        fn eq(&self, other: &LogicalExpr) -> bool {
            match *other {
                LogicalExpr {
                    left: ref __self_1_0,
                    right: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    LogicalExpr {
                        left: ref __self_0_0,
                        right: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LogicalExpr) -> bool {
            match *other {
                LogicalExpr {
                    left: ref __self_1_0,
                    right: ref __self_1_1,
                    operator: ref __self_1_2,
                } => match *self {
                    LogicalExpr {
                        left: ref __self_0_0,
                        right: ref __self_0_1,
                        operator: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl LogicalExpr {
        pub fn new(left: Box<Expr>, right: Box<Expr>, operator: LogicalOperator) -> LogicalExpr {
            LogicalExpr {
                left,
                right,
                operator,
            }
        }
    }
    pub struct ThisExpr {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ThisExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ThisExpr {} => {
                    let mut debug_trait_builder = f.debug_struct("ThisExpr");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ThisExpr {
        #[inline]
        fn clone(&self) -> ThisExpr {
            match *self {
                ThisExpr {} => ThisExpr {},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ThisExpr {
        #[inline]
        fn eq(&self, other: &ThisExpr) -> bool {
            match *other {
                ThisExpr {} => match *self {
                    ThisExpr {} => true,
                },
            }
        }
    }
    impl ThisExpr {
        pub fn new() -> ThisExpr {
            ThisExpr {}
        }
    }
    pub struct VarExpr {
        pub name: Token,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for VarExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                VarExpr {
                    name: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("VarExpr");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for VarExpr {
        #[inline]
        fn clone(&self) -> VarExpr {
            match *self {
                VarExpr {
                    name: ref __self_0_0,
                } => VarExpr {
                    name: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for VarExpr {
        #[inline]
        fn eq(&self, other: &VarExpr) -> bool {
            match *other {
                VarExpr {
                    name: ref __self_1_0,
                } => match *self {
                    VarExpr {
                        name: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &VarExpr) -> bool {
            match *other {
                VarExpr {
                    name: ref __self_1_0,
                } => match *self {
                    VarExpr {
                        name: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl VarExpr {
        pub fn new(name: Token) -> VarExpr {
            VarExpr { name }
        }
    }
    pub struct IdentifierExpr {
        pub value: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for IdentifierExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                IdentifierExpr {
                    value: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("IdentifierExpr");
                    let _ = debug_trait_builder.field("value", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for IdentifierExpr {
        #[inline]
        fn clone(&self) -> IdentifierExpr {
            match *self {
                IdentifierExpr {
                    value: ref __self_0_0,
                } => IdentifierExpr {
                    value: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for IdentifierExpr {
        #[inline]
        fn eq(&self, other: &IdentifierExpr) -> bool {
            match *other {
                IdentifierExpr {
                    value: ref __self_1_0,
                } => match *self {
                    IdentifierExpr {
                        value: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &IdentifierExpr) -> bool {
            match *other {
                IdentifierExpr {
                    value: ref __self_1_0,
                } => match *self {
                    IdentifierExpr {
                        value: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl IdentifierExpr {
        pub fn new(value: String) -> IdentifierExpr {
            IdentifierExpr { value }
        }
    }
    pub struct UnaryExpr {
        pub value: Box<Expr>,
        pub operator: UnaryOperator,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UnaryExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UnaryExpr {
                    value: ref __self_0_0,
                    operator: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("UnaryExpr");
                    let _ = debug_trait_builder.field("value", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("operator", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UnaryExpr {
        #[inline]
        fn clone(&self) -> UnaryExpr {
            match *self {
                UnaryExpr {
                    value: ref __self_0_0,
                    operator: ref __self_0_1,
                } => UnaryExpr {
                    value: ::std::clone::Clone::clone(&(*__self_0_0)),
                    operator: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UnaryExpr {
        #[inline]
        fn eq(&self, other: &UnaryExpr) -> bool {
            match *other {
                UnaryExpr {
                    value: ref __self_1_0,
                    operator: ref __self_1_1,
                } => match *self {
                    UnaryExpr {
                        value: ref __self_0_0,
                        operator: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UnaryExpr) -> bool {
            match *other {
                UnaryExpr {
                    value: ref __self_1_0,
                    operator: ref __self_1_1,
                } => match *self {
                    UnaryExpr {
                        value: ref __self_0_0,
                        operator: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl UnaryExpr {
        pub fn new(value: Box<Expr>, operator: UnaryOperator) -> UnaryExpr {
            UnaryExpr { value, operator }
        }
    }
    pub struct PostfixExpr {
        pub value: Box<Expr>,
        pub operator: PostfixOperator,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for PostfixExpr {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                PostfixExpr {
                    value: ref __self_0_0,
                    operator: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("PostfixExpr");
                    let _ = debug_trait_builder.field("value", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("operator", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PostfixExpr {
        #[inline]
        fn clone(&self) -> PostfixExpr {
            match *self {
                PostfixExpr {
                    value: ref __self_0_0,
                    operator: ref __self_0_1,
                } => PostfixExpr {
                    value: ::std::clone::Clone::clone(&(*__self_0_0)),
                    operator: ::std::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for PostfixExpr {
        #[inline]
        fn eq(&self, other: &PostfixExpr) -> bool {
            match *other {
                PostfixExpr {
                    value: ref __self_1_0,
                    operator: ref __self_1_1,
                } => match *self {
                    PostfixExpr {
                        value: ref __self_0_0,
                        operator: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &PostfixExpr) -> bool {
            match *other {
                PostfixExpr {
                    value: ref __self_1_0,
                    operator: ref __self_1_1,
                } => match *self {
                    PostfixExpr {
                        value: ref __self_0_0,
                        operator: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl PostfixExpr {
        pub fn new(value: Box<Expr>, operator: PostfixOperator) -> PostfixExpr {
            PostfixExpr { value, operator }
        }
    }
}
#[inline]
pub(crate) fn resolve_binary(
    l: ast::Expr,
    mut o: Vec<(ast::BinaryOperator, ast::Expr)>,
) -> ast::Expr {
    if o.is_empty() {
        return l;
    }
    let first = o.pop().unwrap();
    let left = ast::Expr::Binary(ast::BinaryExpr::new(
        Box::new(l),
        Box::new(first.1),
        first.0,
    ));
    if o.is_empty() {
        return left;
    }
    resolve_binary(left, o)
}
#[inline]
pub(crate) fn resolve_logical(
    l: ast::Expr,
    mut o: Vec<(ast::LogicalOperator, ast::Expr)>,
) -> ast::Expr {
    if o.is_empty() {
        return l;
    }
    let first = o.pop().unwrap();
    let left = ast::Expr::Logical(ast::LogicalExpr::new(
        Box::new(l),
        Box::new(first.1),
        first.0,
    ));
    if o.is_empty() {
        return left;
    }
    resolve_logical(left, o)
}
pub mod parser {
    use self::RuleResult::{Failed, Matched};
    use super::ast::*;
    use super::{resolve_binary, resolve_logical};
    fn escape_default(s: &str) -> String {
        s.chars().flat_map(|c| c.escape_default()).collect()
    }
    fn char_range_at(s: &str, pos: usize) -> (char, usize) {
        let c = &s[pos..].chars().next().unwrap();
        let next_pos = pos + c.len_utf8();
        (*c, next_pos)
    }
    enum RuleResult<T> {
        Matched(usize, T),
        Failed,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::std::clone::Clone> ::std::clone::Clone for RuleResult<T> {
        #[inline]
        fn clone(&self) -> RuleResult<T> {
            match (&*self,) {
                (&RuleResult::Matched(ref __self_0, ref __self_1),) => RuleResult::Matched(
                    ::std::clone::Clone::clone(&(*__self_0)),
                    ::std::clone::Clone::clone(&(*__self_1)),
                ),
                (&RuleResult::Failed,) => RuleResult::Failed,
            }
        }
    }
    #[structural_match]
    pub struct ParseError {
        pub line: usize,
        pub column: usize,
        pub offset: usize,
        pub expected: ::std::collections::HashSet<&'static str>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ParseError {
        #[inline]
        fn eq(&self, other: &ParseError) -> bool {
            match *other {
                ParseError {
                    line: ref __self_1_0,
                    column: ref __self_1_1,
                    offset: ref __self_1_2,
                    expected: ref __self_1_3,
                } => match *self {
                    ParseError {
                        line: ref __self_0_0,
                        column: ref __self_0_1,
                        offset: ref __self_0_2,
                        expected: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ParseError) -> bool {
            match *other {
                ParseError {
                    line: ref __self_1_0,
                    column: ref __self_1_1,
                    offset: ref __self_1_2,
                    expected: ref __self_1_3,
                } => match *self {
                    ParseError {
                        line: ref __self_0_0,
                        column: ref __self_0_1,
                        offset: ref __self_0_2,
                        expected: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ParseError {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<usize>;
                let _: ::std::cmp::AssertParamIsEq<usize>;
                let _: ::std::cmp::AssertParamIsEq<usize>;
                let _: ::std::cmp::AssertParamIsEq<::std::collections::HashSet<&'static str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ParseError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ParseError {
                    line: ref __self_0_0,
                    column: ref __self_0_1,
                    offset: ref __self_0_2,
                    expected: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ParseError");
                    let _ = debug_trait_builder.field("line", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("column", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("offset", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("expected", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ParseError {
        #[inline]
        fn clone(&self) -> ParseError {
            match *self {
                ParseError {
                    line: ref __self_0_0,
                    column: ref __self_0_1,
                    offset: ref __self_0_2,
                    expected: ref __self_0_3,
                } => ParseError {
                    line: ::std::clone::Clone::clone(&(*__self_0_0)),
                    column: ::std::clone::Clone::clone(&(*__self_0_1)),
                    offset: ::std::clone::Clone::clone(&(*__self_0_2)),
                    expected: ::std::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    pub type ParseResult<T> = Result<T, ParseError>;
    impl ::std::fmt::Display for ParseError {
        fn fmt(
            &self,
            fmt: &mut ::std::fmt::Formatter,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            fmt.write_fmt(::std::fmt::Arguments::new_v1(
                &["error at ", ":", ": expected "],
                &match (&self.line, &self.column) {
                    (arg0, arg1) => [
                        ::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt),
                        ::std::fmt::ArgumentV1::new(arg1, ::std::fmt::Display::fmt),
                    ],
                },
            ))?;
            if self.expected.len() == 0 {
                fmt.write_fmt(::std::fmt::Arguments::new_v1(
                    &["EOF"],
                    &match () {
                        () => [],
                    },
                ))?;
            } else if self.expected.len() == 1 {
                fmt.write_fmt(::std::fmt::Arguments::new_v1(
                    &["`", "`"],
                    &match (&escape_default(self.expected.iter().next().unwrap()),) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                    },
                ))?;
            } else {
                let mut iter = self.expected.iter();
                fmt.write_fmt(::std::fmt::Arguments::new_v1(
                    &["one of `", "`"],
                    &match (&escape_default(iter.next().unwrap()),) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                    },
                ))?;
                for elem in iter {
                    fmt.write_fmt(::std::fmt::Arguments::new_v1(
                        &[", `", "`"],
                        &match (&escape_default(elem),) {
                            (arg0,) => {
                                [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                            }
                        },
                    ))?;
                }
            }
            Ok(())
        }
    }
    impl ::std::error::Error for ParseError {
        fn description(&self) -> &str {
            "parse error"
        }
    }
    fn slice_eq(
        input: &str,
        state: &mut ParseState,
        pos: usize,
        m: &'static str,
    ) -> RuleResult<()> {
        #![inline]
        #![allow(dead_code)]
        let l = m.len();
        if input.len() >= pos + l && &input.as_bytes()[pos..pos + l] == m.as_bytes() {
            Matched(pos + l, ())
        } else {
            state.mark_failure(pos, m)
        }
    }
    fn slice_eq_case_insensitive(
        input: &str,
        state: &mut ParseState,
        pos: usize,
        m: &'static str,
    ) -> RuleResult<()> {
        #![inline]
        #![allow(dead_code)]
        let mut used = 0usize;
        let mut input_iter = input[pos..].chars().flat_map(|x| x.to_uppercase());
        for m_char_upper in m.chars().flat_map(|x| x.to_uppercase()) {
            used += m_char_upper.len_utf8();
            let input_char_result = input_iter.next();
            if input_char_result.is_none() || input_char_result.unwrap() != m_char_upper {
                return state.mark_failure(pos, m);
            }
        }
        Matched(pos + used, ())
    }
    fn any_char(input: &str, state: &mut ParseState, pos: usize) -> RuleResult<()> {
        #![inline]
        #![allow(dead_code)]
        if input.len() > pos {
            let (_, next) = char_range_at(input, pos);
            Matched(next, ())
        } else {
            state.mark_failure(pos, "<character>")
        }
    }
    fn pos_to_line(input: &str, pos: usize) -> (usize, usize) {
        let before = &input[..pos];
        let line = before.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1;
        let col = before.chars().rev().take_while(|&c| c != '\n').count() + 1;
        (line, col)
    }
    impl<'input> ParseState<'input> {
        fn mark_failure(&mut self, pos: usize, expected: &'static str) -> RuleResult<()> {
            if self.suppress_fail == 0 {
                if pos > self.max_err_pos {
                    self.max_err_pos = pos;
                    self.expected.clear();
                }
                if pos == self.max_err_pos {
                    self.expected.insert(expected);
                }
            }
            Failed
        }
    }
    struct ParseState<'input> {
        max_err_pos: usize,
        suppress_fail: usize,
        expected: ::std::collections::HashSet<&'static str>,
        _phantom: ::std::marker::PhantomData<&'input ()>,
    }
    impl<'input> ParseState<'input> {
        fn new() -> ParseState<'input> {
            ParseState {
                max_err_pos: 0,
                suppress_fail: 0,
                expected: ::std::collections::HashSet::new(),
                _phantom: ::std::marker::PhantomData,
            }
        }
    }
    fn __parse_program<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Vec<Stmt>> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse___(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse_statements(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, s) => {
                            let __seq_res = __parse___(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { s }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_statements<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Vec<Stmt>> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let mut __repeat_pos = __pos;
                let mut __repeat_value = <[_]>::into_vec(box []);
                loop {
                    let __pos = __repeat_pos;
                    let __pos = if __repeat_value.len() > 0 {
                        let __sep_res = __parse___(__input, __state, __pos);
                        match __sep_res {
                            Matched(__newpos, _) => __newpos,
                            Failed => break,
                        }
                    } else {
                        __pos
                    };
                    let __step_res = __parse_statement(__input, __state, __pos);
                    match __step_res {
                        Matched(__newpos, __value) => {
                            __repeat_pos = __newpos;
                            __repeat_value.push(__value);
                        }
                        Failed => {
                            break;
                        }
                    }
                }
                Matched(__repeat_pos, __repeat_value)
            };
            match __seq_res {
                Matched(__pos, s) => Matched(__pos, { s }),
                Failed => Failed,
            }
        }
    }
    fn __parse_statement<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = __parse_block(__input, __state, __pos);
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = __parse_class_decl(__input, __state, __pos);
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = __parse_func_statement(__input, __state, __pos);
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res = __parse_return_stmt(__input, __state, __pos);
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __choice_res =
                                                __parse_var_statement(__input, __state, __pos);
                                            match __choice_res {
                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                Failed => {
                                                    let __choice_res =
                                                        __parse_if_stmt(__input, __state, __pos);
                                                    match __choice_res {
                                                        Matched(__pos, __value) => {
                                                            Matched(__pos, __value)
                                                        }
                                                        Failed => __parse_expression_statement(
                                                            __input, __state, __pos,
                                                        ),
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_class_decl<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_token_class(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let str_start = __pos;
                                match __parse_identifier_raw(__input, __state, __pos) {
                                    Matched(__newpos, _) => {
                                        Matched(__newpos, &__input[str_start..__newpos])
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, name) => {
                                    let __seq_res = __parse___(__input, __state, __pos);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = match {
                                                let __seq_res =
                                                    slice_eq(__input, __state, __pos, "extends");
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res =
                                                            __parse___(__input, __state, __pos);
                                                        match __seq_res {
                                                            Matched(__pos, _) => {
                                                                let __seq_res = {
                                                                    let str_start = __pos;
                                                                    match __parse_identifier_raw(
                                                                        __input, __state, __pos,
                                                                    ) {
                                                                        Matched(__newpos, _) => {
                                                                            Matched(
                                                                                __newpos,
                                                                                &__input[str_start
                                                                                    ..__newpos],
                                                                            )
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                };
                                                                match __seq_res {
                                                                    Matched(__pos, s) => {
                                                                        Matched(__pos, {
                                                                            s.to_string()
                                                                        })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            } {
                                                Matched(__newpos, __value) => {
                                                    Matched(__newpos, Some(__value))
                                                }
                                                Failed => Matched(__pos, None),
                                            };
                                            match __seq_res {
                                                Matched(__pos, extends) => {
                                                    let __seq_res =
                                                        __parse___(__input, __state, __pos);
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res = match {
                                                                let __seq_res = slice_eq(
                                                                    __input,
                                                                    __state,
                                                                    __pos,
                                                                    "implements",
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = __parse___(
                                                                            __input, __state, __pos,
                                                                        );
                                                                        match __seq_res {
                                                                            Matched(__pos, _) => {
                                                                                let __seq_res = {
                                                                                    let mut __repeat_pos =
                                                                                        __pos;
                                                                                    let mut __repeat_value = < [ _ ] > :: into_vec ( box [ ] ) ;
                                                                                    loop {
                                                                                        let __pos = __repeat_pos ;
                                                                                        let __pos = if __repeat_value . len ( ) > 0 { let __sep_res = { let __seq_res = __parse___ ( __input , __state , __pos ) ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = slice_eq ( __input , __state , __pos , "," ) ; match __seq_res { Matched ( __pos , _ ) => { __parse___ ( __input , __state , __pos ) } Failed => Failed , } } } Failed => Failed , } } ; match __sep_res { Matched ( __newpos , _ ) => { __newpos } Failed => break , } } else { __pos } ;
                                                                                        let __step_res = {
                                                                                            let __seq_res = {
                                                                                                let str_start = __pos ;
                                                                                                match __parse_identifier_raw ( __input , __state , __pos ) { Matched ( __newpos , _ ) => { Matched ( __newpos , & __input [ str_start .. __newpos ] ) } Failed => Failed , }
                                                                                            };
                                                                                            match __seq_res { Matched ( __pos , s ) => { Matched ( __pos , { s . to_string ( ) } ) } Failed => Failed , }
                                                                                        };
                                                                                        match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; __repeat_value . push ( __value ) ; } Failed => { break ; } }
                                                                                    }
                                                                                    if __repeat_value . len ( ) >= 1 { Matched ( __repeat_pos , __repeat_value ) } else { Failed }
                                                                                };
                                                                                match __seq_res {
                                                                                    Matched(
                                                                                        __pos,
                                                                                        i,
                                                                                    ) => Matched(
                                                                                        __pos,
                                                                                        { i },
                                                                                    ),
                                                                                    Failed => {
                                                                                        Failed
                                                                                    }
                                                                                }
                                                                            }
                                                                            Failed => Failed,
                                                                        }
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            } {
                                                                Matched(__newpos, __value) => {
                                                                    Matched(__newpos, Some(__value))
                                                                }
                                                                Failed => Matched(__pos, None),
                                                            };
                                                            match __seq_res {
                                                                Matched(__pos, implements) => {
                                                                    let __seq_res = __parse___(
                                                                        __input, __state, __pos,
                                                                    );
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => {
                                                                            let __seq_res =
                                                                                slice_eq(
                                                                                    __input,
                                                                                    __state, __pos,
                                                                                    "{",
                                                                                );
                                                                            match __seq_res {
                                                                                Matched(
                                                                                    __pos,
                                                                                    _,
                                                                                ) => {
                                                                                    let __seq_res =
                                                                                        __parse___(
                                                                                            __input,
                                                                                            __state,
                                                                                            __pos,
                                                                                        );
                                                                                    match __seq_res
                                                                                    {
                                                                                        Matched(
                                                                                            __pos,
                                                                                            _,
                                                                                        ) => {
                                                                                            let __seq_res = {
                                                                                                let mut __repeat_pos = __pos ;
                                                                                                let mut __repeat_value = < [ _ ] > :: into_vec ( box [ ] ) ;
                                                                                                loop {
                                                                                                    let __pos = __repeat_pos ;
                                                                                                    let __pos = if __repeat_value . len ( ) > 0 { let __sep_res = __parse___ ( __input , __state , __pos ) ; match __sep_res { Matched ( __newpos , _ ) => { __newpos } Failed => break , } } else { __pos } ;
                                                                                                    let __step_res = __parse_class_body ( __input , __state , __pos ) ;
                                                                                                    match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; __repeat_value . push ( __value ) ; } Failed => { break ; } }
                                                                                                }
                                                                                                Matched ( __repeat_pos , __repeat_value )
                                                                                            };
                                                                                            match __seq_res { Matched ( __pos , body ) => { { let __seq_res = __parse___ ( __input , __state , __pos ) ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = slice_eq ( __input , __state , __pos , "}" ) ; match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { Stmt :: Class ( ClassStmt :: new ( name . to_string ( ) , body , extends , implements . unwrap_or ( Vec :: new ( ) ) ) ) } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , }
                                                                                        }
                                                                                        Failed => {
                                                                                            Failed
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_class_body<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Box<Stmt>> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_func_statement(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, s) => Matched(__pos, { Box::new(s) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_func_statement<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_token_function(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let str_start = __pos;
                                match __parse_identifier_raw(__input, __state, __pos) {
                                    Matched(__newpos, _) => {
                                        Matched(__newpos, &__input[str_start..__newpos])
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, name) => {
                                    let __seq_res = __parse___(__input, __state, __pos);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = slice_eq(__input, __state, __pos, "(");
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res =
                                                        __parse___(__input, __state, __pos);
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res =
                                                                match __parse_formal_parameter_list(
                                                                    __input, __state, __pos,
                                                                ) {
                                                                    Matched(__newpos, __value) => {
                                                                        Matched(
                                                                            __newpos,
                                                                            Some(__value),
                                                                        )
                                                                    }
                                                                    Failed => Matched(__pos, None),
                                                                };
                                                            match __seq_res {
                                                                Matched(__pos, params) => {
                                                                    let __seq_res = __parse___(
                                                                        __input, __state, __pos,
                                                                    );
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => {
                                                                            let __seq_res =
                                                                                slice_eq(
                                                                                    __input,
                                                                                    __state, __pos,
                                                                                    ")",
                                                                                );
                                                                            match __seq_res {
                                                                                Matched(
                                                                                    __pos,
                                                                                    _,
                                                                                ) => {
                                                                                    let __seq_res =
                                                                                        __parse___(
                                                                                            __input,
                                                                                            __state,
                                                                                            __pos,
                                                                                        );
                                                                                    match __seq_res
                                                                                    {
                                                                                        Matched(
                                                                                            __pos,
                                                                                            _,
                                                                                        ) => {
                                                                                            let __seq_res = __parse_block ( __input , __state , __pos ) ;
                                                                                            match __seq_res { Matched ( __pos , b ) => { { let __seq_res = __parse___ ( __input , __state , __pos ) ; match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { Stmt :: Func ( FuncStmt :: new ( name . to_string ( ) , Box :: new ( b ) , params . unwrap_or ( Vec :: new ( ) ) ) ) } ) } Failed => Failed , } } } Failed => Failed , }
                                                                                        }
                                                                                        Failed => {
                                                                                            Failed
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_formal_parameter_list<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Vec<Argument>> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = __parse_formal_parameter_item_vaargs(__input, __state, __pos);
                match __seq_res {
                    Matched(__pos, a) => Matched(__pos, { <[_]>::into_vec(box [a]) }),
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __seq_res = {
                        let mut __repeat_pos = __pos;
                        let mut __repeat_value = <[_]>::into_vec(box []);
                        loop {
                            let __pos = __repeat_pos;
                            let __pos = if __repeat_value.len() > 0 {
                                let __sep_res = {
                                    let __seq_res = __parse___(__input, __state, __pos);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = slice_eq(__input, __state, __pos, ",");
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    __parse___(__input, __state, __pos)
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                };
                                match __sep_res {
                                    Matched(__newpos, _) => __newpos,
                                    Failed => break,
                                }
                            } else {
                                __pos
                            };
                            let __step_res = __parse_formal_parameter_item(__input, __state, __pos);
                            match __step_res {
                                Matched(__newpos, __value) => {
                                    __repeat_pos = __newpos;
                                    __repeat_value.push(__value);
                                }
                                Failed => {
                                    break;
                                }
                            }
                        }
                        if __repeat_value.len() >= 1 {
                            Matched(__repeat_pos, __repeat_value)
                        } else {
                            Failed
                        }
                    };
                    match __seq_res {
                        Matched(__pos, args) => {
                            let __seq_res = __parse___(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = match __parse_formal_parameter_item_vaargs(
                                        __input, __state, __pos,
                                    ) {
                                        Matched(__newpos, __value) => {
                                            Matched(__newpos, Some(__value))
                                        }
                                        Failed => Matched(__pos, None),
                                    };
                                    match __seq_res {
                                        Matched(__pos, rest) => Matched(__pos, {
                                            let mut a = args;
                                            if rest.is_some() {
                                                a.push(rest.unwrap());
                                            }
                                            a
                                        }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
    fn __parse_formal_parameter_item<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Argument> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match __parse_identifier_raw(__input, __state, __pos) {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, o) => Matched(__pos, { Argument::Regular(o.to_string()) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_formal_parameter_item_vaargs<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Argument> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_SPREAD_OPERATOR(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let str_start = __pos;
                                match __parse_identifier_raw(__input, __state, __pos) {
                                    Matched(__newpos, _) => {
                                        Matched(__newpos, &__input[str_start..__newpos])
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, i) => {
                                    Matched(__pos, { Argument::Rest(i.to_string()) })
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_return_stmt<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_token_return(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = match __parse_expression(__input, __state, __pos) {
                                Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                Failed => Matched(__pos, None),
                            };
                            match __seq_res {
                                Matched(__pos, e) => {
                                    Matched(__pos, { Stmt::Return(ReturnStmt::new(e)) })
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_expression_statement<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse__(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse_expression(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, e) => Matched(__pos, { Stmt::Expr(ExprStmt::new(e)) }),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_var_statement<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_token_var(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __pos = if __repeat_value.len() > 0 {
                                        let __sep_res = {
                                            let __seq_res = __parse___(__input, __state, __pos);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res =
                                                        slice_eq(__input, __state, __pos, ",");
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            __parse___(__input, __state, __pos)
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        };
                                        match __sep_res {
                                            Matched(__newpos, _) => __newpos,
                                            Failed => break,
                                        }
                                    } else {
                                        __pos
                                    };
                                    let __step_res = __parse_var_decl(__input, __state, __pos);
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, vars) => Matched(__pos, {
                                    if vars.len() == 1 {
                                        Stmt::Var(vars.into_iter().nth(0).unwrap())
                                    } else {
                                        Stmt::VarList(VarListStmt::new(vars))
                                    }
                                }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_var_decl<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<VarStmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match __parse_identifier_raw(__input, __state, __pos) {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, i) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = match __parse_initializer(__input, __state, __pos) {
                                Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                Failed => Matched(__pos, None),
                            };
                            match __seq_res {
                                Matched(__pos, init) => {
                                    Matched(__pos, { VarStmt::new(i.to_string(), init) })
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_initializer<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "=");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let __assert_res = slice_eq(__input, __state, __pos, "=");
                        __state.suppress_fail -= 1;
                        match __assert_res {
                            Failed => Matched(__pos, ()),
                            Matched(..) => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse___(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res =
                                        __parse_assignment_expr(__input, __state, __pos);
                                    match __seq_res {
                                        Matched(__pos, a) => Matched(__pos, { a }),
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_block<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "{");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse_statements(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, s) => {
                                    let __seq_res = __parse___(__input, __state, __pos);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = slice_eq(__input, __state, __pos, "}");
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, {
                                                    Stmt::Block(BlockStmt::new(
                                                        s.into_iter()
                                                            .map(|m| Box::new(m))
                                                            .collect(),
                                                    ))
                                                }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_if_stmt<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Stmt> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_token_if(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse_expression(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, test) => {
                                    let __seq_res = __parse___(__input, __state, __pos);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = __parse_block(__input, __state, __pos);
                                            match __seq_res {
                                                Matched(__pos, b) => {
                                                    let __seq_res =
                                                        __parse___(__input, __state, __pos);
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res = match __parse_else_stmt(
                                                                __input, __state, __pos,
                                                            ) {
                                                                Matched(__newpos, __value) => {
                                                                    Matched(__newpos, Some(__value))
                                                                }
                                                                Failed => Matched(__pos, None),
                                                            };
                                                            match __seq_res {
                                                                Matched(__pos, e) => {
                                                                    Matched(__pos, {
                                                                        Stmt::If(IfStmt::new(
                                                                            test,
                                                                            Box::new(b),
                                                                            e,
                                                                        ))
                                                                    })
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_else_stmt<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Box<Stmt>> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_token_else(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse_statement(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, s) => Matched(__pos, { Box::new(s) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_expression<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        __parse_assignment_expr(__input, __state, __pos)
    }
    fn __parse_assignment_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = __parse_left_hand_side(__input, __state, __pos);
                match __seq_res {
                    Matched(__pos, l) => {
                        let __seq_res = __parse___(__input, __state, __pos);
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res =
                                    __parse_assignment_operator(__input, __state, __pos);
                                match __seq_res {
                                    Matched(__pos, o) => {
                                        let __seq_res = __parse___(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = __parse_assignment_expr(
                                                    __input, __state, __pos,
                                                );
                                                match __seq_res {
                                                    Matched(__pos, r) => Matched(__pos, {
                                                        Expr::Assign(AssignExpr::new(
                                                            Box::new(l),
                                                            Box::new(r),
                                                            o,
                                                        ))
                                                    }),
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => __parse_logical_or_expr(__input, __state, __pos),
            }
        }
    }
    fn __parse_left_hand_side<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = __parse_call_expr(__input, __state, __pos);
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => __parse_member_expr(__input, __state, __pos),
            }
        }
    }
    fn __parse_call_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_member_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, m) => {
                    let __seq_res = __parse_arguments(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, a) => {
                            Matched(__pos, { Expr::Call(CallExpr::new(Box::new(m), a)) })
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_arguments<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Vec<Expr>> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "(");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = match {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __pos = if __repeat_value.len() > 0 {
                                        let __sep_res = {
                                            let __seq_res = __parse___(__input, __state, __pos);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res =
                                                        slice_eq(__input, __state, __pos, ",");
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            __parse___(__input, __state, __pos)
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        };
                                        match __sep_res {
                                            Matched(__newpos, _) => __newpos,
                                            Failed => break,
                                        }
                                    } else {
                                        __pos
                                    };
                                    let __step_res =
                                        __parse_logical_or_expr(__input, __state, __pos);
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            } {
                                Matched(__newpos, __value) => Matched(__newpos, Some(__value)),
                                Failed => Matched(__pos, None),
                            };
                            match __seq_res {
                                Matched(__pos, a) => {
                                    let __seq_res = __parse___(__input, __state, __pos);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = slice_eq(__input, __state, __pos, ")");
                                            match __seq_res {
                                                Matched(__pos, _) => Matched(__pos, {
                                                    if a.is_some() {
                                                        a.unwrap()
                                                    } else {
                                                        Vec::new()
                                                    }
                                                }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_member_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = __parse_primary_expr(__input, __state, __pos);
                match __seq_res {
                    Matched(__pos, o) => {
                        let __seq_res = {
                            let __choice_res = {
                                let __seq_res = slice_eq(__input, __state, __pos, "[");
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = __parse_expression(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, e) => {
                                                let __seq_res =
                                                    slice_eq(__input, __state, __pos, "]");
                                                match __seq_res {
                                                    Matched(__pos, _) => Matched(__pos, { e }),
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __seq_res = slice_eq(__input, __state, __pos, ".");
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res =
                                                __parse_identifier(__input, __state, __pos);
                                            match __seq_res {
                                                Matched(__pos, i) => Matched(__pos, { i }),
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                        };
                        match __seq_res {
                            Matched(__pos, p) => Matched(__pos, {
                                Expr::Member(MemberExpr::new(Box::new(o), Box::new(p), false))
                            }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => __parse_primary_expr(__input, __state, __pos),
            }
        }
    }
    fn __parse_primary_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = __parse_this(__input, __state, __pos);
                match __seq_res {
                    Matched(__pos, t) => Matched(__pos, { t }),
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = {
                        let __seq_res = __parse_identifier(__input, __state, __pos);
                        match __seq_res {
                            Matched(__pos, i) => Matched(__pos, { i }),
                            Failed => Failed,
                        }
                    };
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = {
                                let __seq_res = __parse_literal(__input, __state, __pos);
                                match __seq_res {
                                    Matched(__pos, l) => Matched(__pos, { l }),
                                    Failed => Failed,
                                }
                            };
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __seq_res = slice_eq(__input, __state, __pos, "(");
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res =
                                                __parse_expression(__input, __state, __pos);
                                            match __seq_res {
                                                Matched(__pos, e) => {
                                                    let __seq_res =
                                                        slice_eq(__input, __state, __pos, ")");
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, { e }),
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_this<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_token_this(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { Expr::This(ThisExpr::new()) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_postfix_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = __parse_left_hand_side(__input, __state, __pos);
                match __seq_res {
                    Matched(__pos, v) => {
                        let __seq_res = __parse__(__input, __state, __pos);
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = __parse_POSTFIX_OPERATOR(__input, __state, __pos);
                                match __seq_res {
                                    Matched(__pos, o) => {
                                        let __seq_res = __parse___(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, _) => Matched(__pos, {
                                                Expr::Postfix(PostfixExpr::new(Box::new(v), o))
                                            }),
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => __parse_left_hand_side(__input, __state, __pos),
            }
        }
    }
    fn __parse_unary_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = __parse_UNARY_OPERATOR(__input, __state, __pos);
                match __seq_res {
                    Matched(__pos, o) => {
                        let __seq_res = __parse___(__input, __state, __pos);
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = __parse_unary_expr(__input, __state, __pos);
                                match __seq_res {
                                    Matched(__pos, v) => {
                                        let __seq_res = __parse___(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, _) => Matched(__pos, {
                                                Expr::Unary(UnaryExpr::new(Box::new(v), o))
                                            }),
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => __parse_postfix_expr(__input, __state, __pos),
            }
        }
    }
    fn __parse_multiplicative_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_unary_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res = __parse_MULTIPLICATIVE_OPERATOR(
                                            __input, __state, __pos,
                                        );
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_unary_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, e) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, e) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_binary(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_additive_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_multiplicative_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_ADDITIVE_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_multiplicative_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, e) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, e) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_binary(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_shift_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_additive_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_SHIFT_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_additive_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, r) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, r) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_binary(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_relational_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_shift_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_RELATIONAL_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_shift_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, r) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, r) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_binary(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_equality_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_relational_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_EQUALITY_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_relational_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, r) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, r) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_binary(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_bitwise_and_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_equality_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_BITWISE_AND_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_equality_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, r) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, r) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_binary(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_bitwise_xor_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_bitwise_and_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_BITWISE_XOR_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_bitwise_and_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, r) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, r) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_binary(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_bitwise_or_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_bitwise_xor_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_BITWISE_OR_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_bitwise_xor_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, r) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, r) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_binary(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_logical_and_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_bitwise_or_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_LOGICAL_AND_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_bitwise_or_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, r) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, r) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_logical(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_logical_or_expr<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = __parse_logical_and_expr(__input, __state, __pos);
            match __seq_res {
                Matched(__pos, l) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __step_res = {
                                        let __seq_res =
                                            __parse_LOGICAL_OR_OPERATOR(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, o) => {
                                                let __seq_res = __parse___(__input, __state, __pos);
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        let __seq_res = __parse_logical_and_expr(
                                                            __input, __state, __pos,
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, r) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, { (o, r) })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, ops) => Matched(__pos, { resolve_logical(l, ops) }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_literal<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let __choice_res = __parse_literal_boolean(__input, __state, __pos);
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => {
                        let __choice_res = __parse_literal_number(__input, __state, __pos);
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __choice_res = __parse_literal_string(__input, __state, __pos);
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => {
                                        let __choice_res =
                                            __parse_literal_array(__input, __state, __pos);
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                                __parse_literal_object(__input, __state, __pos)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match __seq_res {
                Matched(__pos, lit) => Matched(__pos, { Expr::Literal(LiteralExpr::new(lit)) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_literal_boolean<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Literal> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match {
                    let __choice_res = slice_eq(__input, __state, __pos, "true");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => slice_eq(__input, __state, __pos, "false"),
                    }
                } {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, b) => Matched(__pos, {
                    Literal::Boolean(if b == "true" { true } else { false })
                }),
                Failed => Failed,
            }
        }
    }
    fn __parse_literal_number<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Literal> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let __choice_res = __parse_double(__input, __state, __pos);
                match __choice_res {
                    Matched(__pos, __value) => Matched(__pos, __value),
                    Failed => __parse_int(__input, __state, __pos),
                }
            };
            match __seq_res {
                Matched(__pos, n) => Matched(__pos, { Literal::Number(n) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_int<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Number> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match {
                    let __choice_res = slice_eq(__input, __state, __pos, "0");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __seq_res = if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    '1'...'9' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[1-9]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[1-9]")
                            };
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let mut __repeat_pos = __pos;
                                    loop {
                                        let __pos = __repeat_pos;
                                        let __step_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                '0'...'9' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[0-9]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[0-9]")
                                        };
                                        match __step_res {
                                            Matched(__newpos, __value) => {
                                                __repeat_pos = __newpos;
                                            }
                                            Failed => {
                                                break;
                                            }
                                        }
                                    }
                                    Matched(__repeat_pos, ())
                                }
                                Failed => Failed,
                            }
                        }
                    }
                } {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, i) => Matched(__pos, { Number::Integer(i.parse().unwrap()) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_double<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Number> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match {
                    let __seq_res = {
                        let __choice_res = slice_eq(__input, __state, __pos, "0");
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => {
                                let __seq_res = if __input.len() > __pos {
                                    let (__ch, __next) = char_range_at(__input, __pos);
                                    match __ch {
                                        '1'...'9' => Matched(__next, ()),
                                        _ => __state.mark_failure(__pos, "[1-9]"),
                                    }
                                } else {
                                    __state.mark_failure(__pos, "[1-9]")
                                };
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let mut __repeat_pos = __pos;
                                        loop {
                                            let __pos = __repeat_pos;
                                            let __step_res = if __input.len() > __pos {
                                                let (__ch, __next) = char_range_at(__input, __pos);
                                                match __ch {
                                                    '0'...'9' => Matched(__next, ()),
                                                    _ => __state.mark_failure(__pos, "[0-9]"),
                                                }
                                            } else {
                                                __state.mark_failure(__pos, "[0-9]")
                                            };
                                            match __step_res {
                                                Matched(__newpos, __value) => {
                                                    __repeat_pos = __newpos;
                                                }
                                                Failed => {
                                                    break;
                                                }
                                            }
                                        }
                                        Matched(__repeat_pos, ())
                                    }
                                    Failed => Failed,
                                }
                            }
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = slice_eq(__input, __state, __pos, ".");
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let mut __repeat_pos = __pos;
                                    let mut __repeat_value = <[_]>::into_vec(box []);
                                    loop {
                                        let __pos = __repeat_pos;
                                        let __step_res = if __input.len() > __pos {
                                            let (__ch, __next) = char_range_at(__input, __pos);
                                            match __ch {
                                                '0'...'9' => Matched(__next, ()),
                                                _ => __state.mark_failure(__pos, "[0-9]"),
                                            }
                                        } else {
                                            __state.mark_failure(__pos, "[0-9]")
                                        };
                                        match __step_res {
                                            Matched(__newpos, __value) => {
                                                __repeat_pos = __newpos;
                                                __repeat_value.push(__value);
                                            }
                                            Failed => {
                                                break;
                                            }
                                        }
                                    }
                                    if __repeat_value.len() >= 1 {
                                        Matched(__repeat_pos, ())
                                    } else {
                                        Failed
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                } {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, i) => Matched(__pos, { Number::Double(i.parse().unwrap()) }),
                Failed => Failed,
            }
        }
    }
    fn __parse_literal_string<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Literal> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "\"");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        let str_start = __pos;
                        match {
                            let mut __repeat_pos = __pos;
                            loop {
                                let __pos = __repeat_pos;
                                let __step_res = {
                                    let __choice_res = __parse_raw_string(__input, __state, __pos);
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => __parse_escape(__input, __state, __pos),
                                    }
                                };
                                match __step_res {
                                    Matched(__newpos, __value) => {
                                        __repeat_pos = __newpos;
                                    }
                                    Failed => {
                                        break;
                                    }
                                }
                            }
                            Matched(__repeat_pos, ())
                        } {
                            Matched(__newpos, _) => {
                                Matched(__newpos, &__input[str_start..__newpos])
                            }
                            Failed => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, s) => {
                            let __seq_res = slice_eq(__input, __state, __pos, "\"");
                            match __seq_res {
                                Matched(__pos, _) => {
                                    Matched(__pos, { Literal::String(s.to_string()) })
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_raw_string<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            let mut __repeat_value = <[_]>::into_vec(box []);
            loop {
                let __pos = __repeat_pos;
                let __step_res = {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let __assert_res = {
                            let __choice_res = slice_eq(__input, __state, __pos, "\\");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => slice_eq(__input, __state, __pos, "\""),
                            }
                        };
                        __state.suppress_fail -= 1;
                        match __assert_res {
                            Failed => Matched(__pos, ()),
                            Matched(..) => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => any_char(__input, __state, __pos),
                        Failed => Failed,
                    }
                };
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                        __repeat_value.push(__value);
                    }
                    Failed => {
                        break;
                    }
                }
            }
            if __repeat_value.len() >= 1 {
                Matched(__repeat_pos, ())
            } else {
                Failed
            }
        }
    }
    fn __parse_hex<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = if __input.len() > __pos {
                let (__ch, __next) = char_range_at(__input, __pos);
                match __ch {
                    '0'...'9' => Matched(__next, ()),
                    _ => __state.mark_failure(__pos, "[0-9]"),
                }
            } else {
                __state.mark_failure(__pos, "[0-9]")
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = if __input.len() > __pos {
                        let (__ch, __next) = char_range_at(__input, __pos);
                        match __ch {
                            'a'...'f' => Matched(__next, ()),
                            _ => __state.mark_failure(__pos, "[a-f]"),
                        }
                    } else {
                        __state.mark_failure(__pos, "[a-f]")
                    };
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            if __input.len() > __pos {
                                let (__ch, __next) = char_range_at(__input, __pos);
                                match __ch {
                                    'A'...'F' => Matched(__next, ()),
                                    _ => __state.mark_failure(__pos, "[A-F]"),
                                }
                            } else {
                                __state.mark_failure(__pos, "[A-F]")
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_unicode_hex<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            let mut __repeat_value = <[_]>::into_vec(box []);
            loop {
                let __pos = __repeat_pos;
                if __repeat_value.len() >= 6 {
                    break;
                }
                let __step_res = __parse_hex(__input, __state, __pos);
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                        __repeat_value.push(__value);
                    }
                    Failed => {
                        break;
                    }
                }
            }
            if __repeat_value.len() >= 1 {
                Matched(__repeat_pos, ())
            } else {
                Failed
            }
        }
    }
    fn __parse_predefined<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = slice_eq(__input, __state, __pos, "n");
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = slice_eq(__input, __state, __pos, "r");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "t");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res = slice_eq(__input, __state, __pos, "\\");
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __choice_res =
                                                slice_eq(__input, __state, __pos, "0");
                                            match __choice_res {
                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                Failed => {
                                                    let __choice_res =
                                                        slice_eq(__input, __state, __pos, "\"");
                                                    match __choice_res {
                                                        Matched(__pos, __value) => {
                                                            Matched(__pos, __value)
                                                        }
                                                        Failed => {
                                                            slice_eq(__input, __state, __pos, "\'")
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_byte<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "x");
            match __seq_res {
                Matched(__pos, _) => {
                    let mut __repeat_pos = __pos;
                    let mut __repeat_value = <[_]>::into_vec(box []);
                    loop {
                        let __pos = __repeat_pos;
                        if __repeat_value.len() >= 2 {
                            break;
                        }
                        let __step_res = __parse_hex(__input, __state, __pos);
                        match __step_res {
                            Matched(__newpos, __value) => {
                                __repeat_pos = __newpos;
                                __repeat_value.push(__value);
                            }
                            Failed => {
                                break;
                            }
                        }
                    }
                    if __repeat_value.len() >= 2 {
                        Matched(__repeat_pos, ())
                    } else {
                        Failed
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_unicode<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "u");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = slice_eq(__input, __state, __pos, "{");
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = __parse_unicode_hex(__input, __state, __pos);
                            match __seq_res {
                                Matched(__pos, _) => slice_eq(__input, __state, __pos, "}"),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_escape<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "\\");
            match __seq_res {
                Matched(__pos, _) => {
                    let __choice_res = __parse_predefined(__input, __state, __pos);
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = __parse_byte(__input, __state, __pos);
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => __parse_unicode(__input, __state, __pos),
                            }
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_char_literal<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "\'");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        let __choice_res = __parse_escape(__input, __state, __pos);
                        match __choice_res {
                            Matched(__pos, __value) => Matched(__pos, __value),
                            Failed => any_char(__input, __state, __pos),
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => slice_eq(__input, __state, __pos, "\'"),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_literal_array<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Literal> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "[");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __pos = if __repeat_value.len() > 0 {
                                        let __sep_res = {
                                            let __seq_res = __parse___(__input, __state, __pos);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res =
                                                        slice_eq(__input, __state, __pos, ",");
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            __parse___(__input, __state, __pos)
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        };
                                        match __sep_res {
                                            Matched(__newpos, _) => __newpos,
                                            Failed => break,
                                        }
                                    } else {
                                        __pos
                                    };
                                    let __step_res = __parse_expression(__input, __state, __pos);
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, e) => {
                                    let __seq_res = __parse___(__input, __state, __pos);
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = slice_eq(__input, __state, __pos, "]");
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    Matched(__pos, { Literal::Array(e) })
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_literal_object<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Literal> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "{");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = __parse___(__input, __state, __pos);
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                let mut __repeat_pos = __pos;
                                let mut __repeat_value = <[_]>::into_vec(box []);
                                loop {
                                    let __pos = __repeat_pos;
                                    let __pos = if __repeat_value.len() > 0 {
                                        let __sep_res = {
                                            let __seq_res = __parse___(__input, __state, __pos);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res =
                                                        slice_eq(__input, __state, __pos, ",");
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            __parse___(__input, __state, __pos)
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        };
                                        match __sep_res {
                                            Matched(__newpos, _) => __newpos,
                                            Failed => break,
                                        }
                                    } else {
                                        __pos
                                    };
                                    let __step_res = {
                                        let __choice_res = {
                                            let __seq_res = {
                                                let str_start = __pos;
                                                match __parse_identifier_raw(
                                                    __input, __state, __pos,
                                                ) {
                                                    Matched(__newpos, _) => Matched(
                                                        __newpos,
                                                        &__input[str_start..__newpos],
                                                    ),
                                                    Failed => Failed,
                                                }
                                            };
                                            match __seq_res {
                                                Matched(__pos, k) => {
                                                    let __seq_res =
                                                        __parse___(__input, __state, __pos);
                                                    match __seq_res {
                                                        Matched(__pos, _) => {
                                                            let __seq_res = slice_eq(
                                                                __input, __state, __pos, ":",
                                                            );
                                                            match __seq_res {
                                                                Matched(__pos, _) => {
                                                                    let __seq_res = __parse___(
                                                                        __input, __state, __pos,
                                                                    );
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => {
                                                                            let __seq_res =
                                                                                __parse_expression(
                                                                                    __input,
                                                                                    __state, __pos,
                                                                                );
                                                                            match __seq_res {
                                                                                Matched(
                                                                                    __pos,
                                                                                    v,
                                                                                ) => Matched(
                                                                                    __pos,
                                                                                    {
                                                                                        ObjectEntry :: new ( Expr :: Literal ( LiteralExpr :: new ( Literal :: String ( k . to_string ( ) ) ) ) , v )
                                                                                    },
                                                                                ),
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        };
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                                let __choice_res = {
                                                    let __seq_res = {
                                                        let __choice_res = __parse_literal_string(
                                                            __input, __state, __pos,
                                                        );
                                                        match __choice_res {
                                                            Matched(__pos, __value) => {
                                                                Matched(__pos, __value)
                                                            }
                                                            Failed => __parse_literal_number(
                                                                __input, __state, __pos,
                                                            ),
                                                        }
                                                    };
                                                    match __seq_res {
                                                        Matched(__pos, k) => {
                                                            let __seq_res =
                                                                __parse___(__input, __state, __pos);
                                                            match __seq_res {
                                                                Matched(__pos, _) => {
                                                                    let __seq_res = slice_eq(
                                                                        __input, __state, __pos,
                                                                        ":",
                                                                    );
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => {
                                                                            let __seq_res =
                                                                                __parse___(
                                                                                    __input,
                                                                                    __state, __pos,
                                                                                );
                                                                            match __seq_res {
                                                                                Matched(
                                                                                    __pos,
                                                                                    _,
                                                                                ) => {
                                                                                    let __seq_res = __parse_expression ( __input , __state , __pos ) ;
                                                                                    match __seq_res { Matched ( __pos , v ) => { Matched ( __pos , { ObjectEntry :: new ( Expr :: Literal ( LiteralExpr :: new ( k ) ) , v ) } ) } Failed => Failed , }
                                                                                }
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        }
                                                        Failed => Failed,
                                                    }
                                                };
                                                match __choice_res {
                                                    Matched(__pos, __value) => {
                                                        Matched(__pos, __value)
                                                    }
                                                    Failed => {
                                                        let __seq_res = {
                                                            let __seq_res = slice_eq(
                                                                __input, __state, __pos, "[",
                                                            );
                                                            match __seq_res {
                                                                Matched(__pos, _) => {
                                                                    let __seq_res = __parse___(
                                                                        __input, __state, __pos,
                                                                    );
                                                                    match __seq_res {
                                                                        Matched(__pos, _) => {
                                                                            let __seq_res =
                                                                                __parse_expression(
                                                                                    __input,
                                                                                    __state, __pos,
                                                                                );
                                                                            match __seq_res {
                                                                                Matched(
                                                                                    __pos,
                                                                                    e,
                                                                                ) => {
                                                                                    let __seq_res =
                                                                                        __parse___(
                                                                                            __input,
                                                                                            __state,
                                                                                            __pos,
                                                                                        );
                                                                                    match __seq_res
                                                                                    {
                                                                                        Matched(
                                                                                            __pos,
                                                                                            _,
                                                                                        ) => {
                                                                                            let __seq_res = slice_eq ( __input , __state , __pos , "]" ) ;
                                                                                            match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { e } ) } Failed => Failed , }
                                                                                        }
                                                                                        Failed => {
                                                                                            Failed
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Failed => Failed,
                                                                            }
                                                                        }
                                                                        Failed => Failed,
                                                                    }
                                                                }
                                                                Failed => Failed,
                                                            }
                                                        };
                                                        match __seq_res {
                                                            Matched(__pos, k) => {
                                                                let __seq_res = __parse___(
                                                                    __input, __state, __pos,
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        let __seq_res = slice_eq(
                                                                            __input, __state,
                                                                            __pos, ":",
                                                                        );
                                                                        match __seq_res {
                                                                            Matched(__pos, _) => {
                                                                                let __seq_res =
                                                                                    __parse___(
                                                                                        __input,
                                                                                        __state,
                                                                                        __pos,
                                                                                    );
                                                                                match __seq_res {
                                                                                    Matched(
                                                                                        __pos,
                                                                                        _,
                                                                                    ) => {
                                                                                        let __seq_res = __parse_expression ( __input , __state , __pos ) ;
                                                                                        match __seq_res { Matched ( __pos , v ) => { Matched ( __pos , { ObjectEntry :: new ( k , v ) } ) } Failed => Failed , }
                                                                                    }
                                                                                    Failed => {
                                                                                        Failed
                                                                                    }
                                                                                }
                                                                            }
                                                                            Failed => Failed,
                                                                        }
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            }
                                                            Failed => Failed,
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    match __step_res {
                                        Matched(__newpos, __value) => {
                                            __repeat_pos = __newpos;
                                            __repeat_value.push(__value);
                                        }
                                        Failed => {
                                            break;
                                        }
                                    }
                                }
                                Matched(__repeat_pos, __repeat_value)
                            };
                            match __seq_res {
                                Matched(__pos, entries) => {
                                    let __seq_res = match {
                                        let __seq_res = __parse___(__input, __state, __pos);
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                slice_eq(__input, __state, __pos, ",")
                                            }
                                            Failed => Failed,
                                        }
                                    } {
                                        Matched(__newpos, _) => Matched(__newpos, ()),
                                        Failed => Matched(__pos, ()),
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = __parse___(__input, __state, __pos);
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    let __seq_res =
                                                        slice_eq(__input, __state, __pos, "}");
                                                    match __seq_res {
                                                        Matched(__pos, _) => Matched(__pos, {
                                                            Literal::Object(Object::new(entries))
                                                        }),
                                                        Failed => Failed,
                                                    }
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_token_this<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "this")
    }
    fn __parse_token_function<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "fn")
    }
    fn __parse_token_var<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "let")
    }
    fn __parse_token_return<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "return")
    }
    fn __parse_token_class<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "class")
    }
    fn __parse_token_interface<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "interface")
    }
    fn __parse_token_true<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "true")
    }
    fn __parse_token_false<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "false")
    }
    fn __parse_token_if<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "if")
    }
    fn __parse_token_else<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "else")
    }
    fn __parse_token_for<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "for")
    }
    fn __parse_token_continue<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "continue")
    }
    fn __parse_token_break<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "break")
    }
    fn __parse_token_in<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "in")
    }
    fn __parse_POSTFIX_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<PostfixOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = slice_eq(__input, __state, __pos, "++");
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { PostfixOperator::Increment }),
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __seq_res = slice_eq(__input, __state, __pos, "--");
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { PostfixOperator::Decrement }),
                        Failed => Failed,
                    }
                }
            }
        }
    }
    fn __parse_UNARY_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<UnaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = slice_eq(__input, __state, __pos, "++");
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { UnaryOperator::Increment }),
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = {
                        let __seq_res = slice_eq(__input, __state, __pos, "--");
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { UnaryOperator::Decrement }),
                            Failed => Failed,
                        }
                    };
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = {
                                let __seq_res = slice_eq(__input, __state, __pos, "+");
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = {
                                            __state.suppress_fail += 1;
                                            let __assert_res =
                                                slice_eq(__input, __state, __pos, "=");
                                            __state.suppress_fail -= 1;
                                            match __assert_res {
                                                Failed => Matched(__pos, ()),
                                                Matched(..) => Failed,
                                            }
                                        };
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                Matched(__pos, { UnaryOperator::Plus })
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __seq_res = slice_eq(__input, __state, __pos, "-");
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            let __seq_res = {
                                                __state.suppress_fail += 1;
                                                let __assert_res =
                                                    slice_eq(__input, __state, __pos, "=");
                                                __state.suppress_fail -= 1;
                                                match __assert_res {
                                                    Failed => Matched(__pos, ()),
                                                    Matched(..) => Failed,
                                                }
                                            };
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    Matched(__pos, { UnaryOperator::Minus })
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_ADDITIVE_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<BinaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = slice_eq(__input, __state, __pos, "+");
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = {
                                let __choice_res = slice_eq(__input, __state, __pos, "+");
                                match __choice_res {
                                    Matched(__pos, __value) => Matched(__pos, __value),
                                    Failed => slice_eq(__input, __state, __pos, "="),
                                }
                            };
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { BinaryOperator::Add }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __seq_res = slice_eq(__input, __state, __pos, "-");
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let __assert_res = {
                                    let __choice_res = slice_eq(__input, __state, __pos, "-");
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => slice_eq(__input, __state, __pos, "="),
                                    }
                                };
                                __state.suppress_fail -= 1;
                                match __assert_res {
                                    Failed => Matched(__pos, ()),
                                    Matched(..) => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { BinaryOperator::Sub }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
    fn __parse_MULTIPLICATIVE_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<BinaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = slice_eq(__input, __state, __pos, "*");
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = slice_eq(__input, __state, __pos, "=");
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { BinaryOperator::Mul }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = {
                        let __seq_res = slice_eq(__input, __state, __pos, "/");
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = {
                                    __state.suppress_fail += 1;
                                    let __assert_res = slice_eq(__input, __state, __pos, "=");
                                    __state.suppress_fail -= 1;
                                    match __assert_res {
                                        Failed => Matched(__pos, ()),
                                        Matched(..) => Failed,
                                    }
                                };
                                match __seq_res {
                                    Matched(__pos, _) => Matched(__pos, { BinaryOperator::Div }),
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    };
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __seq_res = slice_eq(__input, __state, __pos, "%");
                            match __seq_res {
                                Matched(__pos, _) => {
                                    let __seq_res = {
                                        __state.suppress_fail += 1;
                                        let __assert_res = slice_eq(__input, __state, __pos, "=");
                                        __state.suppress_fail -= 1;
                                        match __assert_res {
                                            Failed => Matched(__pos, ()),
                                            Matched(..) => Failed,
                                        }
                                    };
                                    match __seq_res {
                                        Matched(__pos, _) => {
                                            Matched(__pos, { BinaryOperator::Mod })
                                        }
                                        Failed => Failed,
                                    }
                                }
                                Failed => Failed,
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_SHIFT_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<BinaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = slice_eq(__input, __state, __pos, "<<");
                match __seq_res {
                    Matched(__pos, _) => {
                        let __seq_res = {
                            __state.suppress_fail += 1;
                            let __assert_res = slice_eq(__input, __state, __pos, "=");
                            __state.suppress_fail -= 1;
                            match __assert_res {
                                Failed => Matched(__pos, ()),
                                Matched(..) => Failed,
                            }
                        };
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { BinaryOperator::ShiftLeft }),
                            Failed => Failed,
                        }
                    }
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __seq_res = slice_eq(__input, __state, __pos, ">>");
                    match __seq_res {
                        Matched(__pos, _) => {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let __assert_res = slice_eq(__input, __state, __pos, "=");
                                __state.suppress_fail -= 1;
                                match __assert_res {
                                    Failed => Matched(__pos, ()),
                                    Matched(..) => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, _) => Matched(__pos, { BinaryOperator::ShiftRight }),
                                Failed => Failed,
                            }
                        }
                        Failed => Failed,
                    }
                }
            }
        }
    }
    fn __parse_RELATIONAL_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<BinaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = slice_eq(__input, __state, __pos, "<=");
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { BinaryOperator::Lte }),
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = {
                        let __seq_res = slice_eq(__input, __state, __pos, ">=");
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { BinaryOperator::Gte }),
                            Failed => Failed,
                        }
                    };
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = {
                                let __seq_res = slice_eq(__input, __state, __pos, "<");
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        let __seq_res = {
                                            __state.suppress_fail += 1;
                                            let __assert_res =
                                                slice_eq(__input, __state, __pos, "<");
                                            __state.suppress_fail -= 1;
                                            match __assert_res {
                                                Failed => Matched(__pos, ()),
                                                Matched(..) => Failed,
                                            }
                                        };
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                Matched(__pos, { BinaryOperator::Lt })
                                            }
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res = {
                                        let __seq_res = slice_eq(__input, __state, __pos, ">");
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                let __seq_res = {
                                                    __state.suppress_fail += 1;
                                                    let __assert_res =
                                                        slice_eq(__input, __state, __pos, ">");
                                                    __state.suppress_fail -= 1;
                                                    match __assert_res {
                                                        Failed => Matched(__pos, ()),
                                                        Matched(..) => Failed,
                                                    }
                                                };
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        Matched(__pos, { BinaryOperator::Gt })
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __seq_res = slice_eq(__input, __state, __pos, "is");
                                            match __seq_res {
                                                Matched(__pos, _) => {
                                                    Matched(__pos, { BinaryOperator::Is })
                                                }
                                                Failed => Failed,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_EQUALITY_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<BinaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = slice_eq(__input, __state, __pos, "==");
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { BinaryOperator::Eq }),
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __seq_res = slice_eq(__input, __state, __pos, "!=");
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::Neq }),
                        Failed => Failed,
                    }
                }
            }
        }
    }
    fn __parse_LOGICAL_AND_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<LogicalOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "&&");
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { LogicalOperator::And }),
                Failed => Failed,
            }
        }
    }
    fn __parse_LOGICAL_OR_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<LogicalOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "||");
            match __seq_res {
                Matched(__pos, _) => Matched(__pos, { LogicalOperator::Or }),
                Failed => Failed,
            }
        }
    }
    fn __parse_assignment_operator<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<AssignmentOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = slice_eq(__input, __state, __pos, "*=");
                match __seq_res {
                    Matched(__pos, _) => Matched(__pos, { AssignmentOperator::Mul }),
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = {
                        let __seq_res = slice_eq(__input, __state, __pos, "/=");
                        match __seq_res {
                            Matched(__pos, _) => Matched(__pos, { AssignmentOperator::Div }),
                            Failed => Failed,
                        }
                    };
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = {
                                let __seq_res = slice_eq(__input, __state, __pos, "%=");
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        Matched(__pos, { AssignmentOperator::Mod })
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res = {
                                        let __seq_res = slice_eq(__input, __state, __pos, "+=");
                                        match __seq_res {
                                            Matched(__pos, _) => {
                                                Matched(__pos, { AssignmentOperator::Add })
                                            }
                                            Failed => Failed,
                                        }
                                    };
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __choice_res = {
                                                let __seq_res =
                                                    slice_eq(__input, __state, __pos, "-=");
                                                match __seq_res {
                                                    Matched(__pos, _) => {
                                                        Matched(__pos, { AssignmentOperator::Sub })
                                                    }
                                                    Failed => Failed,
                                                }
                                            };
                                            match __choice_res {
                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                Failed => {
                                                    let __choice_res = {
                                                        let __seq_res = slice_eq(
                                                            __input, __state, __pos, "<<=",
                                                        );
                                                        match __seq_res {
                                                            Matched(__pos, _) => Matched(__pos, {
                                                                AssignmentOperator::ShiftLeft
                                                            }),
                                                            Failed => Failed,
                                                        }
                                                    };
                                                    match __choice_res {
                                                        Matched(__pos, __value) => {
                                                            Matched(__pos, __value)
                                                        }
                                                        Failed => {
                                                            let __choice_res = {
                                                                let __seq_res = slice_eq(
                                                                    __input, __state, __pos, ">>=",
                                                                );
                                                                match __seq_res {
                                                                    Matched(__pos, _) => {
                                                                        Matched(__pos, {
                                                                            AssignmentOperator :: ShiftRight
                                                                        })
                                                                    }
                                                                    Failed => Failed,
                                                                }
                                                            };
                                                            match __choice_res {
                                                                Matched(__pos, __value) => {
                                                                    Matched(__pos, __value)
                                                                }
                                                                Failed => {
                                                                    let __choice_res = {
                                                                        let __seq_res = slice_eq(
                                                                            __input, __state,
                                                                            __pos, "&=",
                                                                        );
                                                                        match __seq_res {
                                                                            Matched(__pos, _) => {
                                                                                Matched(__pos, {
                                                                                    AssignmentOperator :: BitwiseAnd
                                                                                })
                                                                            }
                                                                            Failed => Failed,
                                                                        }
                                                                    };
                                                                    match __choice_res {
                                                                        Matched(__pos, __value) => {
                                                                            Matched(__pos, __value)
                                                                        }
                                                                        Failed => {
                                                                            let __choice_res = {
                                                                                let __seq_res =
                                                                                    slice_eq(
                                                                                        __input,
                                                                                        __state,
                                                                                        __pos,
                                                                                        "^=",
                                                                                    );
                                                                                match __seq_res {
                                                                                    Matched(
                                                                                        __pos,
                                                                                        _,
                                                                                    ) => Matched(
                                                                                        __pos,
                                                                                        {
                                                                                            AssignmentOperator :: BitwiseXor
                                                                                        },
                                                                                    ),
                                                                                    Failed => {
                                                                                        Failed
                                                                                    }
                                                                                }
                                                                            };
                                                                            match __choice_res {
                                                                                Matched(
                                                                                    __pos,
                                                                                    __value,
                                                                                ) => Matched(
                                                                                    __pos, __value,
                                                                                ),
                                                                                Failed => {
                                                                                    let __choice_res = {
                                                                                        let __seq_res = slice_eq ( __input , __state , __pos , "|=" ) ;
                                                                                        match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { AssignmentOperator :: BitwiseOr } ) } Failed => Failed , }
                                                                                    };
                                                                                    match __choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let __seq_res = { let __seq_res = slice_eq ( __input , __state , __pos , "=" ) ; match __seq_res { Matched ( __pos , _ ) => { { __state . suppress_fail += 1 ; let __assert_res = slice_eq ( __input , __state , __pos , "=" ) ; __state . suppress_fail -= 1 ; match __assert_res { Failed => Matched ( __pos , ( ) ) , Matched ( .. ) => Failed , } } } Failed => Failed , } } ; match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { AssignmentOperator :: Assign } ) } Failed => Failed , } } }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_BITWISE_AND_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<BinaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "&");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let __assert_res = {
                            let __choice_res = slice_eq(__input, __state, __pos, "&");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => slice_eq(__input, __state, __pos, "="),
                            }
                        };
                        __state.suppress_fail -= 1;
                        match __assert_res {
                            Failed => Matched(__pos, ()),
                            Matched(..) => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::BitwiseAnd }),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_BITWISE_XOR_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<BinaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "^");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let __assert_res = slice_eq(__input, __state, __pos, "=");
                        __state.suppress_fail -= 1;
                        match __assert_res {
                            Failed => Matched(__pos, ()),
                            Matched(..) => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::BitwiseXor }),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_BITWISE_OR_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<BinaryOperator> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "|");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let __assert_res = {
                            let __choice_res = slice_eq(__input, __state, __pos, "|");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => slice_eq(__input, __state, __pos, "="),
                            }
                        };
                        __state.suppress_fail -= 1;
                        match __assert_res {
                            Failed => Matched(__pos, ()),
                            Matched(..) => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => Matched(__pos, { BinaryOperator::BitwiseOr }),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_SPREAD_OPERATOR<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        slice_eq(__input, __state, __pos, "...")
    }
    fn __parse_reserved_words<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = __parse_token_this(__input, __state, __pos);
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = __parse_token_function(__input, __state, __pos);
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = __parse_token_var(__input, __state, __pos);
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res =
                                        __parse_token_return(__input, __state, __pos);
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __choice_res =
                                                __parse_token_class(__input, __state, __pos);
                                            match __choice_res {
                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                Failed => {
                                                    let __choice_res =
                                                        __parse_token_true(__input, __state, __pos);
                                                    match __choice_res {
                                                        Matched(__pos, __value) => {
                                                            Matched(__pos, __value)
                                                        }
                                                        Failed => {
                                                            let __choice_res = __parse_token_false(
                                                                __input, __state, __pos,
                                                            );
                                                            match __choice_res {
                                                                Matched(__pos, __value) => {
                                                                    Matched(__pos, __value)
                                                                }
                                                                Failed => {
                                                                    let __choice_res =
                                                                        __parse_token_if(
                                                                            __input, __state, __pos,
                                                                        );
                                                                    match __choice_res {
                                                                        Matched(__pos, __value) => {
                                                                            Matched(__pos, __value)
                                                                        }
                                                                        Failed => {
                                                                            let __choice_res =
                                                                                __parse_token_else(
                                                                                    __input,
                                                                                    __state, __pos,
                                                                                );
                                                                            match __choice_res {
                                                                                Matched(
                                                                                    __pos,
                                                                                    __value,
                                                                                ) => Matched(
                                                                                    __pos, __value,
                                                                                ),
                                                                                Failed => {
                                                                                    let __choice_res = __parse_token_for ( __input , __state , __pos ) ;
                                                                                    match __choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let __choice_res = __parse_token_continue ( __input , __state , __pos ) ; match __choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => { let __choice_res = __parse_token_break ( __input , __state , __pos ) ; match __choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => __parse_token_in ( __input , __state , __pos ) , } } } } }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_identifier<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<Expr> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                let str_start = __pos;
                match {
                    let __seq_res = {
                        __state.suppress_fail += 1;
                        let __assert_res = __parse_reserved_words(__input, __state, __pos);
                        __state.suppress_fail -= 1;
                        match __assert_res {
                            Failed => Matched(__pos, ()),
                            Matched(..) => Failed,
                        }
                    };
                    match __seq_res {
                        Matched(__pos, _) => __parse_identifier_name(__input, __state, __pos),
                        Failed => Failed,
                    }
                } {
                    Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                    Failed => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, i) => Matched(__pos, {
                    Expr::Identifier(IdentifierExpr::new(i.to_string()))
                }),
                Failed => Failed,
            }
        }
    }
    fn __parse_identifier_raw<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = {
                __state.suppress_fail += 1;
                let __assert_res = __parse_reserved_words(__input, __state, __pos);
                __state.suppress_fail -= 1;
                match __assert_res {
                    Failed => Matched(__pos, ()),
                    Matched(..) => Failed,
                }
            };
            match __seq_res {
                Matched(__pos, _) => __parse_identifier_name(__input, __state, __pos),
                Failed => Failed,
            }
        }
    }
    fn __parse_identifier_name<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = if __input.len() > __pos {
                let (__ch, __next) = char_range_at(__input, __pos);
                match __ch {
                    'a'...'z' | 'A'...'Z' => Matched(__next, ()),
                    _ => __state.mark_failure(__pos, "[a-zA-Z]"),
                }
            } else {
                __state.mark_failure(__pos, "[a-zA-Z]")
            };
            match __seq_res {
                Matched(__pos, _) => {
                    let mut __repeat_pos = __pos;
                    loop {
                        let __pos = __repeat_pos;
                        let __step_res = if __input.len() > __pos {
                            let (__ch, __next) = char_range_at(__input, __pos);
                            match __ch {
                                'a'...'z' | 'A'...'Z' => Matched(__next, ()),
                                _ => __state.mark_failure(__pos, "[a-zA-Z]"),
                            }
                        } else {
                            __state.mark_failure(__pos, "[a-zA-Z]")
                        };
                        match __step_res {
                            Matched(__newpos, __value) => {
                                __repeat_pos = __newpos;
                            }
                            Failed => {
                                break;
                            }
                        }
                    }
                    Matched(__repeat_pos, ())
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_whitespace<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = slice_eq(__input, __state, __pos, "\t");
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = slice_eq(__input, __state, __pos, "\\v");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "\\f");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res = slice_eq(__input, __state, __pos, " ");
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __choice_res =
                                                slice_eq(__input, __state, __pos, "\u{a0}");
                                            match __choice_res {
                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                Failed => {
                                                    slice_eq(__input, __state, __pos, "\u{feff}")
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_source_character<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        any_char(__input, __state, __pos)
    }
    fn __parse_line_terminator<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = slice_eq(__input, __state, __pos, "\n");
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = slice_eq(__input, __state, __pos, "\r");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "\u{2028}");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => slice_eq(__input, __state, __pos, "\u{2029}"),
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_line_terminator_sequence<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = slice_eq(__input, __state, __pos, "\n");
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = slice_eq(__input, __state, __pos, "\r\n");
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = slice_eq(__input, __state, __pos, "\r");
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res =
                                        slice_eq(__input, __state, __pos, "\u{2028}");
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => slice_eq(__input, __state, __pos, "\u{2029}"),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn __parse_comment<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = __parse_multi_line_comment(__input, __state, __pos);
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => __parse_single_line_comment(__input, __state, __pos),
            }
        }
    }
    fn __parse_multi_line_comment<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "/*");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        let mut __repeat_pos = __pos;
                        loop {
                            let __pos = __repeat_pos;
                            let __step_res = {
                                let __seq_res = {
                                    __state.suppress_fail += 1;
                                    let __assert_res = slice_eq(__input, __state, __pos, "*/");
                                    __state.suppress_fail -= 1;
                                    match __assert_res {
                                        Failed => Matched(__pos, ()),
                                        Matched(..) => Failed,
                                    }
                                };
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        __parse_source_character(__input, __state, __pos)
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __step_res {
                                Matched(__newpos, __value) => {
                                    __repeat_pos = __newpos;
                                }
                                Failed => {
                                    break;
                                }
                            }
                        }
                        Matched(__repeat_pos, ())
                    };
                    match __seq_res {
                        Matched(__pos, _) => slice_eq(__input, __state, __pos, "*/"),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_multi_line_comment_no_line_terminator<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "/*");
            match __seq_res {
                Matched(__pos, _) => {
                    let __seq_res = {
                        let mut __repeat_pos = __pos;
                        loop {
                            let __pos = __repeat_pos;
                            let __step_res = {
                                let __seq_res = {
                                    __state.suppress_fail += 1;
                                    let __assert_res = {
                                        let __choice_res = slice_eq(__input, __state, __pos, "*/");
                                        match __choice_res {
                                            Matched(__pos, __value) => Matched(__pos, __value),
                                            Failed => {
                                                __parse_line_terminator(__input, __state, __pos)
                                            }
                                        }
                                    };
                                    __state.suppress_fail -= 1;
                                    match __assert_res {
                                        Failed => Matched(__pos, ()),
                                        Matched(..) => Failed,
                                    }
                                };
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        __parse_source_character(__input, __state, __pos)
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __step_res {
                                Matched(__newpos, __value) => {
                                    __repeat_pos = __newpos;
                                }
                                Failed => {
                                    break;
                                }
                            }
                        }
                        Matched(__repeat_pos, ())
                    };
                    match __seq_res {
                        Matched(__pos, _) => slice_eq(__input, __state, __pos, "*/"),
                        Failed => Failed,
                    }
                }
                Failed => Failed,
            }
        }
    }
    fn __parse_single_line_comment<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res = slice_eq(__input, __state, __pos, "//");
            match __seq_res {
                Matched(__pos, _) => {
                    let mut __repeat_pos = __pos;
                    loop {
                        let __pos = __repeat_pos;
                        let __step_res = {
                            let __seq_res = {
                                __state.suppress_fail += 1;
                                let __assert_res = __parse_line_terminator(__input, __state, __pos);
                                __state.suppress_fail -= 1;
                                match __assert_res {
                                    Failed => Matched(__pos, ()),
                                    Matched(..) => Failed,
                                }
                            };
                            match __seq_res {
                                Matched(__pos, _) => {
                                    __parse_source_character(__input, __state, __pos)
                                }
                                Failed => Failed,
                            }
                        };
                        match __step_res {
                            Matched(__newpos, __value) => {
                                __repeat_pos = __newpos;
                            }
                            Failed => {
                                break;
                            }
                        }
                    }
                    Matched(__repeat_pos, ())
                }
                Failed => Failed,
            }
        }
    }
    fn __parse__<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            loop {
                let __pos = __repeat_pos;
                let __step_res = __parse_whitespace(__input, __state, __pos);
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                    }
                    Failed => {
                        break;
                    }
                }
            }
            Matched(__repeat_pos, ())
        }
    }
    fn __parse___<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            loop {
                let __pos = __repeat_pos;
                let __step_res = {
                    let __choice_res = __parse_whitespace(__input, __state, __pos);
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res =
                                __parse_line_terminator_sequence(__input, __state, __pos);
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => __parse_comment(__input, __state, __pos),
                            }
                        }
                    }
                };
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                    }
                    Failed => {
                        break;
                    }
                }
            }
            Matched(__repeat_pos, ())
        }
    }
    fn __parse___c<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let mut __repeat_pos = __pos;
            let mut __repeat_value = <[_]>::into_vec(box []);
            loop {
                let __pos = __repeat_pos;
                let __step_res = {
                    let __choice_res = __parse_whitespace(__input, __state, __pos);
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res =
                                __parse_line_terminator_sequence(__input, __state, __pos);
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    let __choice_res =
                                        __parse_multi_line_comment(__input, __state, __pos);
                                    match __choice_res {
                                        Matched(__pos, __value) => Matched(__pos, __value),
                                        Failed => {
                                            let __choice_res =
                                                __parse_multi_line_comment_no_line_terminator(
                                                    __input, __state, __pos,
                                                );
                                            match __choice_res {
                                                Matched(__pos, __value) => Matched(__pos, __value),
                                                Failed => slice_eq(__input, __state, __pos, ";"),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                };
                match __step_res {
                    Matched(__newpos, __value) => {
                        __repeat_pos = __newpos;
                        __repeat_value.push(__value);
                    }
                    Failed => {
                        break;
                    }
                }
            }
            if __repeat_value.len() >= 1 {
                Matched(__repeat_pos, ())
            } else {
                Failed
            }
        }
    }
    fn __parse_EOS<'input>(
        __input: &'input str,
        __state: &mut ParseState<'input>,
        __pos: usize,
    ) -> RuleResult<()> {
        #![allow(non_snake_case, unused)]
        {
            let __choice_res = {
                let __seq_res = match __parse_single_line_comment(__input, __state, __pos) {
                    Matched(__newpos, _) => Matched(__newpos, ()),
                    Failed => Matched(__pos, ()),
                };
                match __seq_res {
                    Matched(__pos, _) => __parse_line_terminator_sequence(__input, __state, __pos),
                    Failed => Failed,
                }
            };
            match __choice_res {
                Matched(__pos, __value) => Matched(__pos, __value),
                Failed => {
                    let __choice_res = {
                        let __seq_res = __parse__(__input, __state, __pos);
                        match __seq_res {
                            Matched(__pos, _) => {
                                let __seq_res = slice_eq(__input, __state, __pos, ";");
                                match __seq_res {
                                    Matched(__pos, _) => __parse__(__input, __state, __pos),
                                    Failed => Failed,
                                }
                            }
                            Failed => Failed,
                        }
                    };
                    match __choice_res {
                        Matched(__pos, __value) => Matched(__pos, __value),
                        Failed => {
                            let __choice_res = {
                                let __seq_res = __parse__(__input, __state, __pos);
                                match __seq_res {
                                    Matched(__pos, _) => {
                                        __state.suppress_fail += 1;
                                        let __assert_res = slice_eq(__input, __state, __pos, "}");
                                        __state.suppress_fail -= 1;
                                        match __assert_res {
                                            Matched(_, __value) => Matched(__pos, __value),
                                            Failed => Failed,
                                        }
                                    }
                                    Failed => Failed,
                                }
                            };
                            match __choice_res {
                                Matched(__pos, __value) => Matched(__pos, __value),
                                Failed => {
                                    __state.suppress_fail += 1;
                                    let __assert_res = any_char(__input, __state, __pos);
                                    __state.suppress_fail -= 1;
                                    match __assert_res {
                                        Failed => Matched(__pos, ()),
                                        Matched(..) => Failed,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn program<'input>(__input: &'input str) -> ParseResult<Vec<Stmt>> {
        #![allow(non_snake_case, unused)]
        let mut __state = ParseState::new();
        match __parse_program(__input, &mut __state, 0) {
            Matched(__pos, __value) => {
                if __pos == __input.len() {
                    return Ok(__value);
                }
            }
            _ => {}
        }
        let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
        Err(ParseError {
            line: __line,
            column: __col,
            offset: __state.max_err_pos,
            expected: __state.expected,
        })
    }
}
