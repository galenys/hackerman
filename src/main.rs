extern crate termion;
extern crate rand;
extern crate transpose;

use termion::{clear, color};
use rand::{Rng};
use std::{thread, time};
use std::cmp::{min, max};

#[derive(Clone)]
struct Strand {
    contents:  String,
    length:    i32,
    end_index: i32,
    lane:      usize,
}
impl Strand {
    fn update(&mut self, source: &String) {
        self.contents.push(get_random_char(source));
        self.contents   = self.contents[1..].to_string();
        self.end_index += 1;
    }
}

fn get_random_char(source: &String) -> char{
    source.as_bytes()[rand::thread_rng().gen_range(0..source.len())] as char
}
fn clear() {
    println!("{}", clear::All);
}
fn new_strand(length: i32, source: &String) -> Strand {
    Strand {
        contents:  (0..length).map(|_| get_random_char(source)).collect::<String>(),
        length:    length,
        end_index: 0,
        lane:      0,
    }
}
fn blank_string(length: i32) -> String {
    (0..length).map(|_| " ").collect::<String>()
}
fn show_chars_of_display(display: Vec<String>, height: usize) {
    let display_chars      : Vec<Vec<char>> = display
        .iter()
        .map(|x| x.chars().collect())
        .collect();
    let display_rows_chars : Vec<Vec<char>> = (0..height).map(|i| display_chars
                                                              .iter()
                                                              .map(|x| x[i])
                                                              .collect()
                                                             ).collect();
    let display_rows       : Vec<String> = display_rows_chars
        .iter()
        .map(|x| x.into_iter().collect::<String>())
        .collect();
    for row in display_rows {
        println!("{red}{row}{reset}", 
                 row   = row,
                 red   = color::Fg(color::Red),
                 reset = color::Fg(color::Reset),
                 );
    }
}

fn show_strands(strands: &Vec<Vec<Strand>>, height: u16) {
    let mut display : Vec<String> = vec![
        blank_string(height as i32); strands.len() as usize
    ];
    for lane in 0..strands.len() {
        for strand in &strands[lane] {

            let start : i32 = max(0,             strand.end_index - strand.length);
            let end   : i32 = min(height as i32, strand.end_index);

            display[lane] = blank_string(start);
            display[lane].push_str(&strand.contents);
            display[lane].push_str(&blank_string(height as i32 - end));
        }
    }

    show_chars_of_display(display, height as usize);
}

fn main() {
    let (width, height) = match termion::terminal_size() {
        Ok(dims) => dims,
        Err(_)   => panic!("Error: Terminal size unreadable")
    };

    let source  = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*".to_string();
    let delay   = time::Duration::from_millis(50);
    let mut rng = rand::thread_rng();
    let min_gap = 5;

    let mut strands : Vec<Vec<Strand>> = vec![vec![]; width as usize];

    loop {
        if rng.gen_range(0.0..1.0) > 0.7 {
            let lane       = rng.gen_range(0..width);
            let length     = rng.gen_range(5..height-5);
            let mut block  = false;
            for strand in &strands[lane as usize] {
                if strand.lane == lane as usize && strand.end_index - strand.length < min_gap {
                    block = true;
                }
            }
            if !block {
                strands[lane as usize].push(new_strand(length as i32, &source));
            }
        }

        clear();
        show_strands(&strands, height);

        for lane in 0..strands.len() {
            let mut index = 0;
            let mut remove_indices = vec![];
            for strand in &mut strands[lane] {
                index += 1;
                strand.update(&source);
                if strand.end_index - strand.length > height as i32 {
                    remove_indices.push(index);
                }
            }
            for i in remove_indices {
                strands[lane].remove(i-1);
            }
        }

        thread::sleep(delay);
    }
}

