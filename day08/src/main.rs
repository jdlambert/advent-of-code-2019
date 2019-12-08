use std::{fs, slice::Chunks};

fn counts(layer: Vec<u8>, target: char) -> usize {
    layer.iter().filter(|c| **c == target as u8).count()
}

fn part1(layers: Chunks<u8>) -> usize {
    let min_layer = layers
        .min_by(|l1, l2| counts(l1.to_vec(), '0').cmp(&counts(l2.to_vec(), '0')))
        .unwrap();

    counts(min_layer.to_vec(), '1') * counts(min_layer.to_vec(), '2')
}

fn color(pixel: usize, layers: Chunks<u8>) -> u8 {
    *layers
        .map(|layer| layer.get(pixel).unwrap())
        .find(|pix| **pix != '2' as u8)
        .unwrap()
}

fn part2(layers: Chunks<u8>) -> String {
    let mut a = String::new();
    for i in 0..6 {
        for j in 0..25 {
            let color = color(i * 25 + j, layers.clone());
            a.push(if color == '1' as u8 { 'X' } else { ' ' });
        }
        a.push('\n');
    }
    a
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let layers = content.trim().as_bytes().chunks(25 * 6);
    println!("Part 1: {}", part1(layers.clone()));
    println!("Part 2: \n{}", part2(layers.clone()));
}
