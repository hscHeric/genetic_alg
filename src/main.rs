use genetic_alg::slice_chromosome;

mod genetic_alg;

fn print_pop(pop: &[Vec<u8>]) {
    for (i, inner_vec) in pop.iter().enumerate() {
        println!("chromosome: {}: {:?}", i, inner_vec);
    }
}

fn main() {
    let pop = genetic_alg::gen_pop(10, 10);
    print_pop(&pop);
    for value in pop.iter() {
        let slice = slice_chromosome(value, 5);
        print_pop(&slice)
    }
}
