mod moudle;
#[cfg(test)]
mod tests;
mod value;

use crate::{ActError, Result, Vars};
use core::fmt;
use rquickjs::{Context as JsContext, Ctx as JsCtx, FromJs, Runtime as JsRuntime};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::cell::RefCell;

use self::value::ActValue;

pub trait ActModule: Send + Sync {
    fn init<'a>(&self, ctx: &JsCtx<'a>) -> Result<()>;
}

pub struct Enviroment {
    vars: RefCell<Vars>,
    modules: RefCell<Vec<Box<dyn ActModule>>>,
}

impl fmt::Debug for Enviroment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Enviroment").finish()
    }
}

unsafe impl Send for Enviroment {}
unsafe impl Sync for Enviroment {}

impl Enviroment {
    pub fn new() -> Self {
        let mut env = Enviroment {
            modules: RefCell::new(Vec::new()),
            vars: RefCell::new(Vars::new()),
        };
        env.init();
        env
    }

    pub fn register_module<T: ActModule + Clone + 'static>(&self, module: &T) {
        let mut modules = self.modules.borrow_mut();
        modules.push(Box::new(module.clone()));
    }

    pub fn get<T>(&self, name: &str) -> Option<T>
    where
        T: for<'de> Deserialize<'de> + Clone,
    {
        self.vars.borrow().get::<T>(name)
    }

    pub fn set<T>(&self, name: &str, value: T)
    where
        T: Serialize + Clone,
    {
        self.vars.borrow_mut().set(name, value);
    }

    pub fn update<F: FnOnce(&mut Vars)>(&self, f: F) {
        let mut vars = self.vars.borrow_mut();
        f(&mut vars);
    }

    pub fn eval<T>(&self, expr: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let runtime = JsRuntime::new().unwrap();
        let ctx = JsContext::full(&runtime).unwrap();
        ctx.with(|ctx| {
            let modules = self.modules.borrow();
            for m in modules.iter() {
                m.init(&ctx)?;
            }

            let result = ctx.eval::<ActValue, &str>(expr);
            if let Err(rquickjs::Error::Exception) = result {
                let exception = rquickjs::Exception::from_js(&ctx, ctx.catch()).unwrap();
                eprintln!("error: {exception:?}");
                return Err(ActError::Exception {
                    ecode: "".to_string(),
                    message: exception.message().unwrap_or_default(),
                });
            }

            let value = result.map_err(ActError::from)?;
            let ret = serde_json::from_value::<T>(value.into()).map_err(ActError::from)?;
            Ok(ret)
        })
    }
}
