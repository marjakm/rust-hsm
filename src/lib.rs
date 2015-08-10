#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
mod macros;
#[cfg(test)]
mod tests;
use std::fmt;


pub trait Name {
    fn name(&self) -> &'static str;
}

pub trait Initializer {
    fn new() -> Self;
}

pub type ClosureFn<UsrStEnum> = Box<Fn() -> Action<UsrStEnum>>;
impl<UsrStEnum: fmt::Debug> fmt::Debug for ClosureFn<UsrStEnum> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(fmt::Display::fmt("ClosureFn", f));
        Ok(())
    }
}

#[derive(Debug)]
pub enum Action<UsrStEnum: fmt::Debug> {
    Ignore,
    Transition(UsrStEnum),
    Closure(ClosureFn<UsrStEnum>)
}
impl<UsrStEnum: fmt::Debug> fmt::Display for Action<UsrStEnum> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Action::Ignore            => try!(fmt::Display::fmt("Ignore", f)),
            &Action::Closure(_)        => try!(fmt::Display::fmt("Closure", f)),
            &Action::Transition(ref x) => try!(fmt::Display::fmt(&format!("Transition({:?})", x), f)),
        };
        Ok(())
    }
}


#[derive(Debug)]
pub enum Event<UsrEvt> {
    Enter,
    User(UsrEvt),
    Exit
}


pub trait State<UsrEvt, UsrStEnum> where Self: Name {
    fn handle_event(&mut self, evt: Event<UsrEvt>) -> Action<UsrStEnum>;
}
impl<'a, UsrEvt, UsrStEnum> fmt::Debug for &'a State<UsrEvt, UsrStEnum> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(fmt::Debug::fmt(self.name(), f));
        Ok(())
    }
}


pub trait StateLookup<UsrStEnum, UsrEvt> {
    fn lookup(&mut self, typ: &UsrStEnum) -> &mut State<UsrEvt, UsrStEnum>;
}


pub struct StateMachine<UsrStStr, UsrStEnum, UsrEvt> {
    current  : UsrStEnum,
    states   : UsrStStr,
    _phantom : ::std::marker::PhantomData<UsrEvt>
}
impl<UsrStStr, UsrStEnum, UsrEvt> StateMachine<UsrStStr, UsrStEnum, UsrEvt>
    where UsrStStr  : Initializer + StateLookup<UsrStEnum, UsrEvt>,
          UsrStEnum : fmt::Debug,
          UsrEvt    : fmt::Debug {

    pub fn new(initial: UsrStEnum) -> Self {
        StateMachine {
            current  : initial,
            states   : UsrStStr::new(),
            _phantom : ::std::marker::PhantomData
        }
    }

    fn handle_action(&mut self, action: Action<UsrStEnum>) {
        match action {
            Action::Ignore          => {},
            Action::Transition(x)   => {self.current = x}
            Action::Closure(x)      => {self.handle_action(x())}
        }
    }

    pub fn input(&mut self, evt: Event<UsrEvt>) {
        debug!("state:  {:?}", self.current);
        debug!("input:  {:?}", evt);
        let action = {
            let cur_st = self.states.lookup(&self.current);
            cur_st.handle_event(evt)
        };
        debug!("action: {:?}", action);
        self.handle_action(action);

    }
}
