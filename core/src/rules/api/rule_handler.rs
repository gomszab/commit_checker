use std::{marker::PhantomPinned, pin::Pin, rc::Rc};

use crate::{
    handler_impl::{
        ClassChecker, ClassNameChecker, CommentChecker, FunctionJsDocChecker, FunctionNameChecker,
        JsDocTypeChecker, PropertyJsDocChecker, PropertyNameChecker, TypeJsDocChecker,
        TypedefJsDocChecker, UnusedVariableChecker, VarKeywordChecker, VariableJsDocChecker,
        VariableNameChecker,
    },
    rules::api::handler::Handler,
};

pub struct RuleHandler {
    pub handlers: Vec<Rc<dyn Handler>>,
    _pin: PhantomPinned,
}

impl RuleHandler {
    pub fn register_handler(self: &mut Pin<Box<Self>>, handler: Rc<dyn Handler>) {
        let handlers = &mut unsafe { self.as_mut().get_unchecked_mut() }.handlers;
        handlers.push(handler);
    }
}

impl RuleHandler {
    pub(crate) fn new() -> Pin<Box<Self>> {
        let mut instance = Box::pin(Self {
            handlers: Vec::new(),
            _pin: PhantomPinned,
        });
        instance.register_handler(Rc::new(CommentChecker));
        instance.register_handler(Rc::new(VariableJsDocChecker));
        instance.register_handler(Rc::new(TypedefJsDocChecker));
        instance.register_handler(Rc::new(TypeJsDocChecker));
        instance.register_handler(Rc::new(JsDocTypeChecker));
        instance.register_handler(Rc::new(VarKeywordChecker));

        instance.register_handler(Rc::new(VariableNameChecker));
        instance.register_handler(Rc::new(FunctionNameChecker));
        instance.register_handler(Rc::new(FunctionJsDocChecker));
        instance.register_handler(Rc::new(UnusedVariableChecker));
        // TODO Handle multiple files in case of unused functionchecker
        // context.register_handler(Rc::new(UnusedFunctionChecker));
        instance.register_handler(Rc::new(ClassNameChecker));
        instance.register_handler(Rc::new(ClassChecker));
        instance.register_handler(Rc::new(PropertyJsDocChecker));
        instance.register_handler(Rc::new(PropertyNameChecker));
        instance
    }
}
