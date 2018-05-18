// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// The output should warn when a loop label is not used. However, it
// should also deal with the edge cases where a label is shadowed,
// within nested loops

// compile-pass

fn main() {
    'unused_while_label: while 0 == 0 {
        //~^ WARN unused label
    }

    let opt = Some(0);
    'unused_while_let_label: while let Some(_) = opt {
        //~^ WARN unused label
    }

    'unused_for_label: for _ in 0..10 {
        //~^ WARN unused label
    }

    'used_loop_label: loop {
        break 'used_loop_label;
    }

    'used_loop_label_outer_1: for _ in 0..10 {
        'used_loop_label_inner_1: for _ in 0..10 {
            break 'used_loop_label_inner_1;
        }
        break 'used_loop_label_outer_1;
    }

    'used_loop_label_outer_2: for _ in 0..10 {
        'unused_loop_label_inner_2: for _ in 0..10 {
            //~^ WARN unused label
            break 'used_loop_label_outer_2;
        }
    }

    'unused_loop_label_outer_3: for _ in 0..10 {
        //~^ WARN unused label
        'used_loop_label_inner_3: for _ in 0..10 {
            break 'used_loop_label_inner_3;
        }
    }

    // Test breaking many times with the same inner label doesn't break the
    // warning on the outer label
    'many_used_shadowed: for _ in 0..10 {
        //~^ WARN unused label
        'many_used_shadowed: for _ in 0..10 {
            //~^ WARN label name `'many_used_shadowed` shadows a label name that is already in scope
            if 1 % 2 == 0 {
                break 'many_used_shadowed;
            } else {
                break 'many_used_shadowed;
            }
        }
    }

    // This is diverging, so put it at the end so we don't get
    // unreachable_code errors everywhere else
    'unused_loop_label: loop {
        //~^ WARN unused label
    }
}
