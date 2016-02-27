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

#![deny(missing_debug_implementations, trivial_casts, trivial_numeric_casts,
        unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]


#[macro_use] extern crate log;

#[macro_use] mod macros;
pub mod traits;
mod state_machine;

pub use ::traits::*;
pub use ::state_machine::StateMachine;

use std::fmt;

#[derive(Debug)]
pub enum Action<UsrStEnum: fmt::Debug> {
    Ignore,
    Parent,
    DelayedTransition,
    Transition(UsrStEnum),
}

#[derive(Debug)]
pub enum Event<'a, UsrEvtEnum, EvtData> where
    UsrEvtEnum: 'a+fmt::Debug,
    EvtData:    'a+fmt::Debug,
{
    Enter(&'a mut EvtData),
    User(&'a mut EvtData, &'a mut UsrEvtEnum),
    Exit(&'a mut EvtData),
}

#[derive(Debug)]
enum EnterOrExit {
    Enter,
    Exit,
}

#[derive(Debug)]
struct Task<UsrStEnum: fmt::Debug>{
    state:          UsrStEnum,
    enter_or_exit:  EnterOrExit,
}

impl<UsrStEnum: fmt::Debug> Task<UsrStEnum> {
    fn new(state: UsrStEnum, enter_or_exit: EnterOrExit) -> Self {
        Task { state: state, enter_or_exit: enter_or_exit }
    }

    fn event<'a, UsrEvtEnum, EvtData>(&self, data: &'a mut EvtData) -> Event<'a, UsrEvtEnum, EvtData> where
        UsrEvtEnum: fmt::Debug,
        EvtData:    fmt::Debug,
    {
        match self.enter_or_exit {
            EnterOrExit::Enter => Event::Enter(data),
            EnterOrExit::Exit  => Event::Exit(data),
        }
    }
}

impl<'a, UsrEvtEnum, UsrStEnum, UsrShrData, EvtData> fmt::Debug for &'a State<UsrEvtEnum, UsrStEnum, UsrShrData, EvtData> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(fmt::Debug::fmt(self.name(), f));
        Ok(())
    }
}
