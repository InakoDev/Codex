// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Or do, no one likes Windows anyway.

fn main() {
    codex_lib::run()
}
