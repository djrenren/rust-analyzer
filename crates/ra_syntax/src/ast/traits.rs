//! Various traits that are implemented by ast nodes.
//!
//! The implementations are usually trivial, and live in generated.rs

use crate::{
    ast::{self, support, AstChildren, AstNode, AstToken},
    syntax_node::SyntaxElementChildren,
    SyntaxToken, T,
};

pub trait TypeAscriptionOwner: AstNode {
    fn ascribed_type(&self) -> Option<ast::TypeRef> {
        support::child(self.syntax())
    }
}

pub trait NameOwner: AstNode {
    fn name(&self) -> Option<ast::Name> {
        support::child(self.syntax())
    }
}

pub trait VisibilityOwner: AstNode {
    fn visibility(&self) -> Option<ast::Visibility> {
        support::child(self.syntax())
    }
}

pub trait LoopBodyOwner: AstNode {
    fn loop_body(&self) -> Option<ast::BlockExpr> {
        support::child(self.syntax())
    }

    fn label(&self) -> Option<ast::Label> {
        support::child(self.syntax())
    }
}

pub trait ArgListOwner: AstNode {
    fn arg_list(&self) -> Option<ast::ArgList> {
        support::child(self.syntax())
    }
}

pub trait ModuleItemOwner: AstNode {
    fn items(&self) -> AstChildren<ast::ModuleItem> {
        support::children(self.syntax())
    }
}

pub trait TypeParamsOwner: AstNode {
    fn type_param_list(&self) -> Option<ast::TypeParamList> {
        support::child(self.syntax())
    }

    fn where_clause(&self) -> Option<ast::WhereClause> {
        support::child(self.syntax())
    }
}

pub trait TypeBoundsOwner: AstNode {
    fn type_bound_list(&self) -> Option<ast::TypeBoundList> {
        support::child(self.syntax())
    }

    fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(self.syntax(), T![:])
    }
}

pub trait AttrsOwner: AstNode {
    fn attrs(&self) -> AstChildren<ast::Attr> {
        support::children(self.syntax())
    }
    fn has_atom_attr(&self, atom: &str) -> bool {
        self.attrs().filter_map(|x| x.as_simple_atom()).any(|x| x == atom)
    }

    fn doc_comments(&self) -> CommentIter {
        CommentIter { iter: self.syntax().children_with_tokens() }
    }
}

pub struct CommentIter {
    iter: SyntaxElementChildren,
}

impl Iterator for CommentIter {
    type Item = ast::Comment;
    fn next(&mut self) -> Option<ast::Comment> {
        self.iter.by_ref().find_map(|el| el.into_token().and_then(ast::Comment::cast))
    }
}
