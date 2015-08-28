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

#[macro_export]
macro_rules! hsm_define_objects_noparents {
    ($st_str:ident, $st_en:ident, $st_evt:ty, $shr_dat:ident, ( $($s:ident),* ) ) => {
        hsm_define_objects!($st_str, $st_en, $st_evt, $shr_dat, ($($s),*) );
        hsm_state_parents!($st_en ; $($s -> None),*);
    };
    ($st_str:ident, $st_en:ident, $st_evt:ty, $shr_dat:ident, ( $($s:ident $x:tt),*)) => {
        hsm_define_objects!($st_str, $st_en, $st_evt, $shr_dat, ($($s $x),*) );
        hsm_state_parents!($st_en ; $($s -> None),*);
    }
}

#[macro_export]
macro_rules! hsm_define_objects {
    ($st_str:ident, $st_en:ident, $st_evt:ty, $shr_dat:ident, ( $($s:ident),* ) ) => {
        use $crate::Parent;
        _hsm_create_states!($($s),*);
        _hsm_create_state_enum!($st_en, ($($s),*));
        _hsm_create_state_struct!($st_str, $st_en, $st_evt, $shr_dat, ($($s),*) );
    };
    ($st_str:ident, $st_en:ident, $st_evt:ty, $shr_dat:ident, ( $($s:ident $x:tt),*)) => {
        use $crate::Parent;
        _hsm_create_states!( $($s $x),* );
        _hsm_create_state_enum!($st_en, ($($s),*));
        _hsm_create_state_struct!($st_str, $st_en, $st_evt, $shr_dat, ($($s),*) );
    }
}

#[macro_export]
macro_rules! hsm_delayed_transition {
    ($probe:ident, $x:block) => {
        match $probe {
            true  => $crate::Action::DelayedTransition,
            false => $crate::Action::Transition($x)
        }
    }
}

#[macro_export]
macro_rules! hsm_impl_state {
    ($state:ident, $events:ident, $states:ident, $shr_data:ident,
     $($pat:pat => $result:expr),*) => {
        impl<'a, 'b, 'c, 'd, 'e> $crate::State<$events, $states, $shr_data> for $state {
            #[allow(unused_variables)]
            fn handle_event(&mut self, shr_data: &mut $shr_data, evt: &$crate::Event<$events>, probe: bool) -> $crate::Action<$states> {
                match *evt {
                    $( $pat => $result),*
                }
            }
        }
    };
    ($state:ident, $events:ident, $states:ident, $shr_data:ident,
     $shr:ident, $evt:ident, $probe:ident, $($pat:pat => $result:expr),*) => {
        impl<'a, 'b, 'c, 'd, 'e> $crate::State<$events, $states, $shr_data> for $state {
            #[allow(unused_variables)]
            fn handle_event(&mut self, $shr: &mut $shr_data, $evt: &$crate::Event<$events>, $probe: bool) -> $crate::Action<$states> {
                match *$evt {
                    $( $pat => $result),*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! hsm_state_parents {
    ($st_en:ident ; $($nam:ident -> $parent:ident),*) => {
        $(_hsm_impl_state_parent!($st_en ; $nam -> $parent);)*
    }
}

#[macro_export]
macro_rules! _hsm_impl_state_parent {
    ($st_en:ident ; $nam:ident -> None) => {
        impl $crate::Parent<$st_en> for $nam {
            fn get_parent() -> Option<$st_en> { None }
        }
    };
    ($st_en:ident ; $nam:ident -> $parent:ident) => {
        impl $crate::Parent<$st_en> for $nam {
            fn get_parent() -> Option<$st_en> { Some($st_en::$parent) }
        }
    }
}

#[macro_export]
macro_rules! _hsm_create_states {
    ( $($s:ident),* ) => {
        $(_hsm_create_state!($s);)*
    };
    ($($s:ident $x:tt),*) => {
        $(_hsm_create_state!($s $x);)*
    }
}

#[macro_export]
macro_rules! _hsm_create_state_common {
    ($nam:ident) => {
        impl $crate::Name for $nam {
            fn name(&self) -> &'static str {
                stringify!($nam)
            }
        }
    }
}

#[macro_export]
macro_rules! _hsm_create_state {
    ($nam:ident) => {
        #[derive(Debug)]
        pub struct $nam;
        impl $crate::Initializer for $nam {
            fn new() -> Self {
                $nam
            }
        }
        _hsm_create_state_common!($nam);
    };
    ($nam:ident { $($field_name:ident : $field_type:ty = $field_default:expr),* }) => {
        #[derive(Debug)]
        pub struct $nam {
            _phantom        : ::std::marker::PhantomData<u8>,
            $( $field_name  : $field_type ),*
        }
        impl $crate::Initializer for $nam {
            fn new() -> Self {
                $nam {
                    _phantom        : ::std::marker::PhantomData,
                    $( $field_name  : $field_default ),*
                }
            }
        }
        _hsm_create_state_common!($nam);
    }
}

#[macro_export]
macro_rules! _hsm_create_state_enum {
    ($st_en:ident, ($($s:ident),*) ) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub enum $st_en {
            $( $s ),*
        }
        impl ::std::fmt::Display for $st_en {
            fn fmt(&self, f:&mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                match *self {
                    $( $st_en::$s => try!(::std::fmt::Display::fmt(stringify!($s), f)) ),*
                };
                Ok(())
            }
        }
        impl $crate::InstanceParent<$st_en> for $st_en {
            fn get_parent(&self) -> Option<$st_en> {
                match *self {
                    $( $st_en::$s => $s::get_parent() ),*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! _hsm_create_state_struct {
    ($st_str:ident, $st_en:ident, $st_evt:ty, $shr_dat:ident, ($($s:ident),*) ) => {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub struct $st_str {
            $( $s : $s ),*
        }
        impl $crate::Initializer for $st_str {
            fn new() -> Self {
                $st_str {
                    $( $s : $s::new() ),*
                }
            }
        }
        impl<'a, 'b, 'c, 'd, 'e> $crate::StateLookup<$st_en, $st_evt, $shr_dat> for $st_str {
            fn lookup(&mut self, typ: &$st_en) -> &mut $crate::State<$st_evt, $st_en, $shr_dat> {
                match *typ {
                    $($st_en::$s => &mut self.$s ),*
                }
            }
        }
    }
}
