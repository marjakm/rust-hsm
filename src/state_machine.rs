/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2016 Mattis Marjak (mattis.marjak@gmail.com)
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

use std::fmt;
use ::traits::*;
use ::{Task, Action, Event, EnterOrExit};


#[derive(Debug)]
pub struct StateMachine<UsrStStr, UsrStEnum, UsrShrData>
    where UsrStStr:   fmt::Debug,
          UsrStEnum:  fmt::Debug,
          UsrShrData: fmt::Debug,
{
    current     : UsrStEnum,
    started     : bool,
    states      : UsrStStr,
    shr_data    : UsrShrData,
    exit_tasks  : Vec<Task<UsrStEnum>>,
    enter_tasks : Vec<Task<UsrStEnum>>,
}

impl<UsrStStr, UsrStEnum, UsrShrData> StateMachine<UsrStStr, UsrStEnum, UsrShrData>
    where UsrStStr   : fmt::Debug + Initializer,
          UsrStEnum  : fmt::Debug + Eq + Clone + InstanceParent<UsrStEnum>,
          UsrShrData : fmt::Debug,
{
    pub fn new(initial: UsrStEnum, shared_data: UsrShrData) -> Self {
        StateMachine {
            current     : initial,
            started     : false,
            states      : UsrStStr::new(),
            shr_data    : shared_data,
            exit_tasks  : Vec::new(),
            enter_tasks : Vec::new(),
        }
    }

    pub fn data(&self) -> &UsrShrData {
        &self.shr_data
    }

    pub fn data_mut(&mut self) -> &mut UsrShrData {
        &mut self.shr_data
    }

    pub fn start<UsrEvtEnum, EvtData>(&mut self, data: &mut EvtData) where
        UsrEvtEnum: fmt::Debug,
        EvtData:    fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData, EvtData>
    {
        let mut parent = Some(self.current.clone());
        while let Some(state) = parent {
            parent = state.get_parent();
            self.enter_tasks.push(Task::new(state, EnterOrExit::Enter));
        }
        self.process_enter_tasks(data);
        self.started = true;
    }

    fn process_exit_tasks<UsrEvtEnum, EvtData>(&mut self, data: &mut EvtData) where
        UsrEvtEnum: fmt::Debug,
        EvtData:    fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData, EvtData>
    {
        for task in self.exit_tasks.iter_mut() {
            debug!("send {:?} to {:?}", task.enter_or_exit, task.state);
            match self.states.lookup(&task.state).handle_event(
                  &mut self.shr_data, &mut task.event(data), false){
                Action::Ignore | Action::Parent => {},
                _ => panic!("Transitions from exit events are not allowed, \
                            ignoring transition from state {:?} on event {:?}",
                            task.state, task.enter_or_exit)
            };
        }
        self.exit_tasks.clear();
    }

    fn process_enter_tasks<UsrEvtEnum, EvtData>(&mut self, data: &mut EvtData) where
        UsrEvtEnum: fmt::Debug,
        EvtData:    fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData, EvtData>
    {
        self.enter_tasks.reverse();
        for task in self.enter_tasks.iter_mut() {
            debug!("send {:?} to {:?}", task.enter_or_exit, task.state);
            match self.states.lookup(&task.state).handle_event(
                  &mut self.shr_data, &mut task.event(data), false){
                Action::Ignore | Action::Parent => {},
                _ => panic!("Transitions from enter events are not allowed, \
                            ignoring transition from state {:?} on event {:?}",
                            task.state, task.enter_or_exit)
            }
        }
        self.enter_tasks.clear();
    }

    fn transition<UsrEvtEnum, EvtData>(&mut self, from_state: UsrStEnum, to_state: UsrStEnum, data: &mut EvtData) where
        UsrEvtEnum: fmt::Debug,
        EvtData:    fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData, EvtData>
    {
        let mut parent = Some(from_state);
        while let Some(state) = parent {
            parent = state.get_parent();
            self.exit_tasks.push(Task::new(state, EnterOrExit::Exit));
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
            self.enter_tasks.push(Task::new(state, EnterOrExit::Enter));
        }
        if let Some(i) = same_idx {
            let drop_num = self.exit_tasks.len() - i;
            for _ in 0..drop_num {
                self.exit_tasks.pop();
            }
        }
        self.process_exit_tasks(data);
        self.process_enter_tasks(data);
    }

    pub fn input<UsrEvtEnum, EvtData>(&mut self, evt: &mut UsrEvtEnum, data: &mut EvtData) where
        UsrEvtEnum: fmt::Debug,
        EvtData:    fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData, EvtData>
    {
        assert!(self.started, "Can't call input before starting the state machine with start()");
        debug!("state:  {:?}", self.current);
        debug!("input:  {:?}", Event::User(data, evt));
        let mut action;
        let mut state = self.current.clone();
        loop {
            action = self.states.lookup(&state).handle_event(&mut self.shr_data, &mut Event::User(data, evt), true);
            match action {
                Action::Ignore               => {
                    self.exit_tasks.clear();
                    break;
                },
                Action::Parent               => {
                    if let Some(parent) = state.get_parent() {
                        self.exit_tasks.push(Task::new(state.clone(), EnterOrExit::Exit));
                        state = parent;
                    } else {
                        panic!("State {:?} responded with Action::Parent to event {:?}, but the state has no parent", state, Event::User(data, evt));
                        // break;
                    }
                },
                Action::Transition(x)        => {
                    debug!("send {:?} to {:?}", Event::User(data, evt), state);
                    self.process_exit_tasks(data);  // exit until in the parent that handles the signal
                    self.current = x.clone(); // signal allready handled
                    self.transition(state.clone(), x, data);
                    break;
                },
                Action::DelayedTransition => {
                    self.process_exit_tasks(data); // exit until in the parent that handles the signal
                    debug!("send {:?} to {:?}", Event::User(data, evt), state);
                    if let Action::Transition(x) = self.states.lookup(&state).handle_event(&mut self.shr_data, &mut Event::User(data, evt), false) { // handle the signal
                        self.current = x.clone();
                        self.transition(state.clone(), x, data);
                    } else {
                        panic!("State {:?} probed Action::DelayedTransition to event {:?}, but doesn't return Action::Transition", state, Event::User(data, evt));
                        // self.current = state;
                    }
                    break;
                },
            }
        }
    }
}
