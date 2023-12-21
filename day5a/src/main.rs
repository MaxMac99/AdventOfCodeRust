#![feature(test)]
extern crate test;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "input.txt";
    let (seeds, maps) = day5a::parse_file(filename)?;
    let seeds = create_seeds(&seeds).collect();

    let result = day5a::find_min_solution(&seeds, &maps);

    println!("Result: {}", result);

    Ok(())
}

fn create_seeds(values: &Vec<u64>) -> impl Iterator<Item=(u64, u64)> + '_ {
    values.iter()
        .map(|chunk| (*chunk, 1))
}


#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works() {
        let filename = "input.txt";
        let (seeds, maps) = day5a::parse_file(filename).unwrap();
        let seeds = create_seeds(&seeds).collect();

        assert_eq!(579439039, day5a::find_min_solution(&seeds, &maps));
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let filename = "input.txt";
        let (seeds, maps) = day5a::parse_file(filename).unwrap();
        let seeds = create_seeds(&seeds).collect();

        b.iter(|| day5a::find_min_solution(&seeds, &maps));
    }
}
