/*
 *  Copyright © 2020 Gustavo Noronha Silva <gustavo@noronha.eti.br>
 *
 *  This file is part of gkrslog.
 *
 *  gkrslog is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  gkrslog is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with gkrslog.  If not, see <http://www.gnu.org/licenses/>.
 */

use ansi_term;
use std::str::FromStr;
use std::io::{BufRead,BufReader};
use structopt::StructOpt;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Yellow,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let lower = src.to_lowercase();
        match lower.as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            _ => Err(String::from("choose one of: red, green, yellow"))
        }
    }
}

fn parse_rule(src: &str) -> Result<(Color, Regex), String> {
    if let Some(pos) = src.find('=') {
        let color = Color::from_str(&src[..pos])?;

        match Regex::new(&src[pos + 1..]) {
            Ok(regex) => Ok((color, regex)),
            Err(e) => Err(e.to_string())
        }
    } else {
        Err(format!("invalid key=value: no `=` found in `{}`", src))
    }    
}

#[derive(Debug, StructOpt)]
#[structopt(name = "gkrslog", about = "Colorize textual input based on regular expressions")]
struct Opt {
    #[structopt(short, long, parse(try_from_str = parse_rule), number_of_values = 1)]
    rule: Vec<(Color, Regex)>,
}

fn try_match_for_line(text: &String, opts: &Opt) -> Option<Color> {
    for rule in &opts.rule {
        if rule.1.is_match(text) {
            return Some(rule.0)
        }
    }
    None
}

fn main() {
    let opts = Opt::from_args();

    let reader = BufReader::new(std::io::stdin()).lines();
    for line in reader {
        let text = line.expect("Failed to read line from stdin.");

        match try_match_for_line(&text, &opts) {
            Some(Color::Red) => {
                println!("{}", ansi_term::Color::Red.paint(text));
            },
            Some(Color::Green) => {
                println!("{}", ansi_term::Color::Green.paint(text));
            },
            Some(Color::Yellow) => {
                println!("{}", ansi_term::Color::Yellow.paint(text));
            },
            _ => { println!("{}", text); }
        }
    }
}