// -*- rust -*-
use std;
import std::task;
import std::task::*;

fn main() {
    let other = task::_spawn(bind child());
    log_err "1";
    yield();
    log_err "2";
    yield();
    log_err "3";
    join_id(other);
}

fn child() { log_err "4"; yield(); log_err "5"; yield(); log_err "6"; }