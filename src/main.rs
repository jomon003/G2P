// g2p — streaming grapheme-to-phoneme CLI built on espeak-ng.
// Copyright (C) 2026  Jomon
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Protocol: one JSON object per line on stdin, one per line on stdout.
//!
//!   in:  {"text":"Bonjour tout le monde.","lang":"fr"}
//!   out: {"phonemes":"bɔ̃ʒˈuʁ tˈu lə mˈɔ̃d"}
//!   err: {"error":"<message>"}
//!
//! `lang` is an espeak-ng voice/language identifier (`en-us`, `es`,
//! `fr`, `it`, `pt-br`, `hi`, …). Output phonemes are espeak-ng's IPA,
//! words joined with single spaces, exactly as espeak produces them —
//! callers do their own post-processing.
//!
//! espeak-ng locates its data directory (the directory CONTAINING
//! `espeak-ng-data`) via the `PIPER_ESPEAKNG_DATA_DIRECTORY`
//! environment variable, falling back to the compiled-in default.
//! Note: espeak-ng has an internal path buffer of roughly 160 bytes —
//! keep the data path short.
//!
//! The process is long-running by design: initialize once, then one
//! request per line until stdin closes. espeak-ng keeps global state,
//! so run one request at a time per process.

use std::io::{BufRead, Write};

#[derive(serde::Deserialize)]
struct Request {
    text: String,
    lang: String,
}

#[derive(serde::Serialize)]
struct Ok_ {
    phonemes: String,
}

#[derive(serde::Serialize)]
struct Err_ {
    error: String,
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match serde_json::from_str::<Request>(&line) {
            Ok(req) => match espeak_rs::text_to_phonemes(&req.text, &req.lang, None) {
                Ok(words) => serde_json::to_string(&Ok_ {
                    phonemes: words.join(" "),
                })
                .unwrap(),
                Err(e) => serde_json::to_string(&Err_ {
                    error: format!("{e:?}"),
                })
                .unwrap(),
            },
            Err(e) => serde_json::to_string(&Err_ {
                error: format!("bad request: {e}"),
            })
            .unwrap(),
        };
        if writeln!(out, "{reply}").and_then(|_| out.flush()).is_err() {
            break;
        }
    }
}
