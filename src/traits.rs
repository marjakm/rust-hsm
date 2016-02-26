use std::fmt;
use ::{Event, Action};


pub trait Name {
    fn name(&self) -> &'static str;
}

pub trait Initializer {
    fn new() -> Self;
}

pub trait Parent<UsrStEnum> {
    fn get_parent() -> Option<UsrStEnum>;
}

pub trait InstanceParent<UsrStEnum> {
    fn get_parent(&self) -> Option<UsrStEnum>;
}

pub trait State<UsrEvtEnum, UsrStEnum, UsrShrData>
    where Self: Name,
          UsrEvtEnum: fmt::Debug,
          UsrStEnum:  fmt::Debug,
          UsrShrData: fmt::Debug,
{
    fn handle_event(&mut self, shr_data: &mut UsrShrData, evt: &Event<UsrEvtEnum>, probe: bool) -> Action<UsrStEnum>;
}

pub trait StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData> {
    fn lookup(&mut self, typ: &UsrStEnum) -> &mut State<UsrEvtEnum, UsrStEnum, UsrShrData>;
}
