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

pub enum Action<UsrStEnum: fmt::Debug> {
    Ignore,
    Parent,
    Transition(UsrStEnum),
    ClosureTransition(Box<Fn() -> UsrStEnum>)
}
impl<UsrStEnum: fmt::Debug> fmt::Debug for Action<UsrStEnum> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Action::Ignore               => try!(fmt::Debug::fmt("Ignore", f)),
            &Action::Parent               => try!(fmt::Debug::fmt("Parent", f)),
            &Action::Transition(ref x)    => try!(fmt::Debug::fmt(&format!("Transition({:?})", x), f)),
            &Action::ClosureTransition(_) => try!(fmt::Debug::fmt("ClosureTransition", f)),
        };
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub enum Event<UsrEvtEnum: Clone> {
    Enter,
    User(UsrEvtEnum),
    Exit
}

pub trait Parent<UsrStEnum> {
    fn get_parent(&self) -> Option<UsrStEnum>;
    fn set_parent(&mut self, newparent: UsrStEnum);
}

pub trait State<UsrEvtEnum, UsrStEnum>
    where Self: Name+Parent<UsrStEnum> {
    fn handle_event(&mut self, evt: Event<UsrEvtEnum>) -> Action<UsrStEnum>;
}
impl<'a, UsrEvtEnum, UsrStEnum> fmt::Debug for &'a State<UsrEvtEnum, UsrStEnum> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(fmt::Debug::fmt(self.name(), f));
        Ok(())
    }
}


pub trait StateLookup<UsrStEnum, UsrEvtEnum> {
    fn lookup(&mut self, typ: &UsrStEnum) -> &mut State<UsrEvtEnum, UsrStEnum>;
}

#[derive(Clone)]
struct Task<UsrStEnum, UsrEvtEnum: Clone> {
    state:  UsrStEnum,
    event: Event<UsrEvtEnum>
}
impl<UsrStEnum, UsrEvtEnum: Clone> Task<UsrStEnum, UsrEvtEnum> {
    fn new(state:  UsrStEnum, event: Event<UsrEvtEnum>) -> Self {
        Task { state:  state, event: event }
    }
}

pub struct StateMachine<UsrStStr, UsrStEnum, UsrEvtEnum: Clone> {
    current  : UsrStEnum,
    states   : UsrStStr,
    tasklist : Vec<Task<UsrStEnum, UsrEvtEnum>>,
    _phantom : ::std::marker::PhantomData<UsrEvtEnum>
}
impl<UsrStStr, UsrStEnum, UsrEvtEnum> StateMachine<UsrStStr, UsrStEnum, UsrEvtEnum>
    where UsrStStr   : Initializer + StateLookup<UsrStEnum, UsrEvtEnum>,
          UsrStEnum  : fmt::Debug+Eq+Clone,
          UsrEvtEnum : fmt::Debug+Clone {

    pub fn new(initial: UsrStEnum) -> Self {
        StateMachine {
            current  : initial,
            states   : UsrStStr::new(),
            tasklist : Vec::new(),
            _phantom : ::std::marker::PhantomData
        }
    }

    fn process_tasklist(&mut self) {
        for task in self.tasklist.iter() {
            debug!("send {:?} to {:?}", task.event, task.state);
            self.states.lookup(&task.state).handle_event(task.event.clone());
        }
        self.tasklist.clear();
    }

    fn transition(&mut self, from_state: UsrStEnum, to_state: UsrStEnum) {
        let mut parent = Some(from_state);
        while let Some(state) = parent {
            parent = self.states.lookup(&state).get_parent();
            self.tasklist.push(Task::new(state, Event::Exit));
        }
        let mut same_idx: Option<usize> = None;
        let mut enter_list = Vec::new();
        parent = Some(to_state);
        'outer: while let Some(state) = parent {
            for (i, task) in self.tasklist.iter().enumerate() {
                if task.state == state {
                    same_idx = Some(i);
                    break 'outer;
                }
            }
            parent = self.states.lookup(&state).get_parent();
            enter_list.push(Task::new(state, Event::Enter));
        }
        if let Some(i) = same_idx {
            let drop_num = self.tasklist.len() - i;
            for _ in 0..drop_num {
                self.tasklist.pop();
            }
        }
        self.process_tasklist();
        enter_list.reverse();
        for task in enter_list.iter() {
            debug!("send {:?} to {:?}", task.event, task.state);
            self.states.lookup(&task.state).handle_event(task.event.clone());
        }
    }

    pub fn input(&mut self, evt: Event<UsrEvtEnum>) {
        debug!("state:  {:?}", self.current);
        debug!("input:  {:?}", evt);
        let mut action;
        let mut state = self.current.clone();
        loop {
            action = self.states.lookup(&state).handle_event(evt.clone());
            match action {
                Action::Ignore               => {
                    self.tasklist.clear();
                    break
                },
                Action::Parent               => {
                    if let Some(parent) = self.states.lookup(&state).get_parent() {
                        self.tasklist.push(Task::new(state.clone(), Event::Exit));
                        state = parent;
                    } else {
                        error!("State {:?} responded with Action::Parent to event {:?}, but the state has no parent", state, evt);
                    }
                },
                Action::Transition(x)        => {
                    debug!("send {:?} to {:?}", evt, state);
                    self.process_tasklist();  // exit until in the parent that handles the signal
                    self.current = x.clone(); // signal allready handled
                    self.transition(state.clone(), x);
                    break;
                },
                Action::ClosureTransition(x) => {
                    self.process_tasklist(); // exit until in the parent that handles the signal
                    debug!("send {:?} to {:?}", evt, state);
                    self.current = x();      // handle the signal
                    let current_state = self.current.clone();
                    self.transition(state.clone(), current_state);
                    break;
                },
            }
        }
    }
}
