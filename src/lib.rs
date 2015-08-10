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


#[derive(Debug)]
pub enum Action<'a, UsrEvt: fmt::Debug> {
    Ignore,
    Transition(&'a State<'a, UsrEvt>)
}
impl<'a, UsrEvt: fmt::Debug> fmt::Display for Action<'a, UsrEvt> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Action::Ignore        => try!(fmt::Display::fmt("Ignore", f)),
            Action::Transition(x) => try!(fmt::Display::fmt(&format!("Transition({:?})", x), f)),
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


pub trait State<'a, UsrEvt> where Self: Name {
    fn handle_event(&'a mut self, evt: Event<UsrEvt>) -> Action<'a, UsrEvt>;
}
impl<'a, UsrEvt> fmt::Debug for &'a State<'a, UsrEvt> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(fmt::Debug::fmt(self.name(), f));
        Ok(())
    }
}


pub trait StateLookup<UsrStEnum, UsrEvt> {
    fn lookup(&mut self, typ: &UsrStEnum) -> &mut State<UsrEvt>;
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

    pub fn input(&mut self, evt: Event<UsrEvt>) {
        info!("input:  {:?}", evt);
        info!("state:  {:?}", self.current);
        let cur_st = self.states.lookup(&self.current);
        let action = cur_st.handle_event(evt);
        info!("action: {:?}", action)

    }
}
