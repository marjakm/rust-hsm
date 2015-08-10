#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
mod macros;
#[cfg(test)]
mod tests;
use std::fmt;


trait Name {
    fn name(&self) -> &'static str;
}

trait Initializer {
    fn new() -> Self;
}


#[derive(Debug)]
enum Action<'a, UsrEvt: fmt::Debug> {
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
enum Event<UsrEvt> {
    Enter,
    User(UsrEvt),
    Exit
}


trait State<'a, UsrEvt> where Self: Name {
    fn handle_event(&'a mut self, evt: Event<UsrEvt>) -> Action<'a, UsrEvt>;
}
impl<'a, UsrEvt> fmt::Debug for &'a State<'a, UsrEvt> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(fmt::Debug::fmt(self.name(), f));
        Ok(())
    }
}


trait StateLookup<UsrStEnum, UsrEvt> {
    fn lookup(&mut self, typ: &UsrStEnum) -> &mut State<UsrEvt>;
}


struct StateMachine<UsrStStr, UsrStEnum, UsrEvt> {
    current  : UsrStEnum,
    states   : UsrStStr,
    _phantom : ::std::marker::PhantomData<UsrEvt>
}
impl<UsrStStr, UsrStEnum, UsrEvt> StateMachine<UsrStStr, UsrStEnum, UsrEvt>
    where UsrStStr  : Initializer + StateLookup<UsrStEnum, UsrEvt>,
          UsrStEnum : fmt::Debug,
          UsrEvt    : fmt::Debug {

    fn new(initial: UsrStEnum) -> Self {
        StateMachine {
            current  : initial,
            states   : UsrStStr::new(),
            _phantom : ::std::marker::PhantomData
        }
    }

    fn input(&mut self, evt: Event<UsrEvt>) {
        info!("input:  {:?}", evt);
        info!("state:  {:?}", self.current);
        let cur_st = self.states.lookup(&self.current);
        let action = cur_st.handle_event(evt);
        info!("action: {:?}", action)

    }
}
