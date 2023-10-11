use crate::parsed::visitor::VisitOrder;

use super::*;

impl<Expr: ExpressionVisitable<Expr>> ExpressionVisitable<Expr> for Identity<Expr> {
    fn visit_expressions_mut<F, B>(&mut self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&mut Expr) -> ControlFlow<B>,
    {
        self.left
            .selector
            .as_mut()
            .into_iter()
            .chain(self.left.expressions.iter_mut())
            .chain(self.right.selector.as_mut())
            .chain(self.right.expressions.iter_mut())
            .try_for_each(move |item| item.visit_expressions_mut(f, o))
    }

    fn visit_expressions<F, B>(&self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&Expr) -> ControlFlow<B>,
    {
        self.left
            .selector
            .as_ref()
            .into_iter()
            .chain(self.left.expressions.iter())
            .chain(self.right.selector.iter())
            .chain(self.right.expressions.iter())
            .try_for_each(move |item| item.visit_expressions(f, o))
    }
}

impl<T> ExpressionVisitable<Expression<T>> for FunctionValueDefinition<T> {
    fn visit_expressions_mut<F, B>(&mut self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&mut Expression<T>) -> ControlFlow<B>,
    {
        match self {
            FunctionValueDefinition::Mapping(e)
            | FunctionValueDefinition::Query(e)
            | FunctionValueDefinition::Expression(e) => e.visit_expressions_mut(f, o),
            FunctionValueDefinition::Array(array) => array
                .iter_mut()
                .flat_map(|a| a.pattern.iter_mut())
                .try_for_each(move |item| item.visit_expressions_mut(f, o)),
        }
    }

    fn visit_expressions<F, B>(&self, f: &mut F, o: VisitOrder) -> ControlFlow<B>
    where
        F: FnMut(&Expression<T>) -> ControlFlow<B>,
    {
        match self {
            FunctionValueDefinition::Mapping(e)
            | FunctionValueDefinition::Query(e)
            | FunctionValueDefinition::Expression(e) => e.visit_expressions(f, o),
            FunctionValueDefinition::Array(array) => array
                .iter()
                .flat_map(|a| a.pattern().iter())
                .try_for_each(move |item| item.visit_expressions(f, o)),
        }
    }
}