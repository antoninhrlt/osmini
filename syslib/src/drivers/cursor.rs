// This file is part of "osmini"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

extern "C" {
    fn _enable();
    fn _disable();
}

/// Wrap with a real Rust function because `_enable` is just a "section"
pub unsafe fn enable() {
    _enable();
}

/// Wrap with a real Rust function because `_disable` is just a "section"
pub unsafe fn disable() {
    _disable();
}
