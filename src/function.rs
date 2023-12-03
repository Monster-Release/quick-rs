use crate::{
    error::QuickError,
    value::{self, Exception, JSValueRef, JS_MKVAL_real},
};
use anyhow::Result;
use quickjs_sys as sys;

pub struct Function {
    value: JSValueRef,
}

impl Function {
    pub fn new(value: JSValueRef) -> Result<Self> {
        Ok(Function { value })
    }

    pub fn call(&self, args: Vec<JSValueRef>) -> Result<JSValueRef, QuickError> {
        let args: Vec<_> = args.iter().map(|arg| arg.val).collect();

        let value = unsafe {
            let undefined = JS_MKVAL_real(sys::JS_TAG_UNDEFINED, 0);

            let result = sys::JS_Call(
                self.value.ctx,
                self.value.val,
                undefined,
                args.len() as _,
                args.as_ptr() as _,
            );

            value::JS_FreeValue_real(self.value.ctx, undefined);

            result
        };

        let value = JSValueRef::from_js_value(self.value.ctx, value);
        if value.is_exception() {
            let value = unsafe { sys::JS_GetException(self.value.ctx) };
            let value = JSValueRef::from_js_value(self.value.ctx, value);

            Err(QuickError::CallError(Exception(value).to_string()))
        } else {
            Ok(value)
        }
    }
}
