#![allow(dead_code)]

#[macro_use]
extern crate log;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

use std::fmt;


macro_rules! impl_fmt_trait_for_action {
    ( $tr:ty, $frmtr:path ) => {
        impl<'a, UsrEvt: fmt::Debug> $tr for Action<'a, UsrEvt> {
            fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
                match *self {
                    Action::Ignore        => try!($frmtr("Ignore", f)),
                    Action::Transition(x) => try!($frmtr(&format!("Transition({:?})", x), f)),
                };
                Ok(())
            }
        }
    }
}
enum Action<'a, UsrEvt: fmt::Debug> {
    Ignore,
    Transition(&'a State<'a, UsrEvt>)
}
impl_fmt_trait_for_action!(fmt::Display, fmt::Display::fmt);
impl_fmt_trait_for_action!(fmt::Debug,   fmt::Debug::fmt);


#[derive(Debug)]
enum Event<UsrEvt> {
    Enter,
    User(UsrEvt),
    Exit
}

trait Name {
    fn name(&self) -> &'static str;
}

trait Initializer {
    fn new() -> Self;
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

trait StateLookup<StEnum, UsrEvt> {
    fn lookup(&mut self, typ: &StEnum) -> &mut State<UsrEvt>;
}

struct StateMachine<UsrEvt, UsrStEnum, UsrStStr> {
    current  : UsrStEnum,
    states   : UsrStStr,
    _phantom : ::std::marker::PhantomData<UsrEvt>
}
impl<UsrEvt, UsrStEnum, UsrStStr> StateMachine<UsrEvt, UsrStEnum, UsrStStr>
    where UsrEvt    : fmt::Debug,
          UsrStEnum : fmt::Debug,
          UsrStStr  : Initializer + StateLookup<UsrStEnum, UsrEvt> {

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
