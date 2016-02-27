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

pub trait State<UsrEvtEnum, UsrStEnum, UsrShrData, EvtData>
    where Self: Name,
          UsrEvtEnum: fmt::Debug,
          UsrStEnum:  fmt::Debug,
          UsrShrData: fmt::Debug,
          EvtData:    fmt::Debug,
{
    fn handle_event(&mut self, shr_data: &mut UsrShrData, evt: &mut Event<UsrEvtEnum, EvtData>, probe: bool) -> Action<UsrStEnum>;
}

pub trait StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData, EvtData> {
    fn lookup(&mut self, typ: &UsrStEnum) -> &mut State<UsrEvtEnum, UsrStEnum, UsrShrData, EvtData>;
}
