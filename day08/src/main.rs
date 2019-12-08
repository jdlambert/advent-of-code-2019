use std::{fs, slice::Chunks};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const BLACK: u8 = '1' as u8;
const WHITE: u8 = '0' as u8;
const TRANSLUCENT: u8 = '2' as u8;

fn counts(layer: Vec<u8>, target: u8) -> usize {
    layer.iter().filter(|c| **c == target).count()
}

fn part1(layers: Chunks<u8>) -> usize {
    let min_layer = layers
        .min_by(|l1, l2| counts(l1.to_vec(), BLACK).cmp(&counts(l2.to_vec(), BLACK)))
        .unwrap();

    counts(min_layer.to_vec(), WHITE) * counts(min_layer.to_vec(), TRANSLUCENT)
}

fn color(pixel: usize, layers: Chunks<u8>) -> u8 {
    *layers
        .map(|layer| layer.get(pixel).unwrap())
        .find(|pix| **pix != TRANSLUCENT)
        .unwrap()
}

fn part2(layers: Chunks<u8>) -> String {
    let mut a = String::new();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let color = color(i * WIDTH + j, layers.clone());
            a.push(if color == BLACK { 'X' } else { ' ' });
        }
        a.push('\n');
    }
    a
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let layers = content.trim().as_bytes().chunks(WIDTH * HEIGHT);
    println!("Part 1: {}", part1(layers.clone()));
    println!("Part 2: \n{}", part2(layers.clone()));
}
