/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Mattis Marjak (mattis.marjak@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
mod macros;
use std::fmt;


pub trait Name {
    fn name(&self) -> &'static str;
}

pub trait Initializer {
    fn new() -> Self;
}

#[derive(Debug)]
pub enum Action<UsrStEnum: fmt::Debug> {
    Ignore,
    Parent,
    DelayedTransition,
    Transition(UsrStEnum),
}

#[derive(Debug, Clone)]
pub enum Event<UsrEvtEnum: Clone> {
    Enter,
    User(UsrEvtEnum),
    Exit
}

pub trait Parent<UsrStEnum> {
    fn get_parent() -> Option<UsrStEnum>;
}

pub trait InstanceParent<UsrStEnum> {
    fn get_parent(&self) -> Option<UsrStEnum>;
}

pub trait State<UsrEvtEnum, UsrStEnum, UsrShrData>
    where Self: Name {
    fn handle_event(&mut self, shr_data: &mut UsrShrData, evt: Event<UsrEvtEnum>, probe: bool) -> Action<UsrStEnum>;
}
impl<'a, UsrEvtEnum, UsrStEnum, UsrShrData> fmt::Debug for &'a State<UsrEvtEnum, UsrStEnum, UsrShrData> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(fmt::Debug::fmt(self.name(), f));
        Ok(())
    }
}


pub trait StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData> {
    fn lookup(&mut self, typ: &UsrStEnum) -> &mut State<UsrEvtEnum, UsrStEnum, UsrShrData>;
}

#[derive(Clone)]
struct Task<UsrStEnum, UsrEvtEnum: Clone> {
    state: UsrStEnum,
    event: Event<UsrEvtEnum>
}
impl<UsrStEnum, UsrEvtEnum: Clone> Task<UsrStEnum, UsrEvtEnum> {
    fn new(state:  UsrStEnum, event: Event<UsrEvtEnum>) -> Self {
        Task { state:  state, event: event }
    }
}

pub struct StateMachine<UsrStStr, UsrStEnum, UsrEvtEnum: Clone, UsrShrData> {
    current     : UsrStEnum,
    started     : bool,
    states      : UsrStStr,
    shr_data    : UsrShrData,
    exit_tasks  : Vec<Task<UsrStEnum, UsrEvtEnum>>,
    enter_tasks : Vec<Task<UsrStEnum, UsrEvtEnum>>,
    _phantom    : ::std::marker::PhantomData<UsrEvtEnum>
}
impl<UsrStStr, UsrStEnum, UsrEvtEnum, UsrShrData> StateMachine<UsrStStr, UsrStEnum, UsrEvtEnum, UsrShrData>
    where UsrStStr   : Initializer + StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData>,
          UsrStEnum  : fmt::Debug+Eq+Clone+InstanceParent<UsrStEnum>,
          UsrEvtEnum : fmt::Debug+Clone {

    pub fn new(initial: UsrStEnum, shared_data: UsrShrData) -> Self {
        StateMachine {
            current     : initial,
            started     : false,
            states      : UsrStStr::new(),
            shr_data    : shared_data,
            exit_tasks  : Vec::new(),
            enter_tasks : Vec::new(),
            _phantom    : ::std::marker::PhantomData
        }
    }

    pub fn start(&mut self) {
        let mut parent = Some(self.current.clone());
        while let Some(state) = parent {
            parent = state.get_parent();
            self.enter_tasks.push(Task::new(state, Event::Enter));
        }
        self.process_enter_tasks();
        self.started = true;
    }

    fn process_exit_tasks(&mut self) {
        for task in self.exit_tasks.iter() {
            debug!("send {:?} to {:?}", task.event, task.state);
            match self.states.lookup(&task.state).handle_event(
                  &mut self.shr_data, task.event.clone(), false){
                Action::Ignore | Action::Parent => {},
                _ => panic!("Transitions from exit events are not allowed, \
                            ignoring transition from state {:?} on event {:?}",
                            task.state, task.event)
            };
        }
        self.exit_tasks.clear();
    }

    fn process_enter_tasks(&mut self) {
        self.enter_tasks.reverse();
        for task in self.enter_tasks.iter() {
            debug!("send {:?} to {:?}", task.event, task.state);
            match self.states.lookup(&task.state).handle_event(
                  &mut self.shr_data, task.event.clone(), false){
                Action::Ignore | Action::Parent => {},
                _ => panic!("Transitions from enter events are not allowed, \
                            ignoring transition from state {:?} on event {:?}",
                            task.state, task.event)
            }
        }
        self.enter_tasks.clear();
    }

    fn transition(&mut self, from_state: UsrStEnum, to_state: UsrStEnum) {
        let mut parent = Some(from_state);
        while let Some(state) = parent {
            parent = state.get_parent();
            self.exit_tasks.push(Task::new(state, Event::Exit));
        }
        let mut same_idx: Option<usize> = None;
        parent = Some(to_state);
        'outer: while let Some(state) = parent {
            for (i, task) in self.exit_tasks.iter().enumerate() {
                if task.state == state {
                    same_idx = Some(i);
                    break 'outer;
                }
            }
            parent = state.get_parent();
            self.enter_tasks.push(Task::new(state, Event::Enter));
        }
        if let Some(i) = same_idx {
            let drop_num = self.exit_tasks.len() - i;
            for _ in 0..drop_num {
                self.exit_tasks.pop();
            }
        }
        self.process_exit_tasks();
        self.process_enter_tasks();
    }

    pub fn input(&mut self, evt: UsrEvtEnum) {
        assert!(self.started, "Can't call input before starting the state machine with start()");
        let evt = Event::User(evt);
        debug!("state:  {:?}", self.current);
        debug!("input:  {:?}", evt);
        let mut action;
        let mut state = self.current.clone();
        loop {
            action = self.states.lookup(&state).handle_event(&mut self.shr_data, evt.clone(), true);
            match action {
                Action::Ignore               => {
                    self.exit_tasks.clear();
                    break;
                },
                Action::Parent               => {
                    if let Some(parent) = state.get_parent() {
                        self.exit_tasks.push(Task::new(state.clone(), Event::Exit));
                        state = parent;
                    } else {
                        panic!("State {:?} responded with Action::Parent to event {:?}, but the state has no parent", state, evt);
                        break;
                    }
                },
                Action::Transition(x)        => {
                    debug!("send {:?} to {:?}", evt, state);
                    self.process_exit_tasks();  // exit until in the parent that handles the signal
                    self.current = x.clone(); // signal allready handled
                    self.transition(state.clone(), x);
                    break;
                },
                Action::DelayedTransition => {
                    self.process_exit_tasks(); // exit until in the parent that handles the signal
                    debug!("send {:?} to {:?}", evt, state);
                    if let Action::Transition(x) = self.states.lookup(&state).handle_event(&mut self.shr_data, evt.clone(), false) { // handle the signal
                        self.current = x.clone();
                        self.transition(state.clone(), x);
                    } else {
                        panic!("State {:?} probed Action::DelayedTransition to event {:?}, but doesn't return Action::Transition", state, evt);
                        self.current = state;
                    }
                    break;
                },
            }
        }
    }
}
