use std::vec;

use rand::Rng;

fn gen_chromosome(num_genes: usize) -> Vec<u8> {
    let mut chromosome = Vec::with_capacity(num_genes);
    let mut rng = rand::thread_rng();
    for _ in 0..num_genes {
        let gene = rng.gen_range(0..=1);
        chromosome.push(gene)
    }

    chromosome
}

pub fn gen_pop(num_genes: usize, num_chromosome: usize) -> Vec<Vec<u8>> {
    let mut pop: Vec<Vec<u8>> = Vec::with_capacity(num_chromosome);
    for _ in 0..num_chromosome {
        let chromosome = gen_chromosome(num_genes);
        pop.push(chromosome);
    }
    pop
}

pub fn slice_chromosome(chromosome: &[u8], slice_len: usize) -> Vec<Vec<u8>> {
    let mut group = Vec::with_capacity(slice_len);
    let mut subgroups = Vec::new();
    for (i, &value) in chromosome.iter().enumerate() {
        group.push(value);
        if (i + 1) % slice_len == 0 {
            subgroups.push(group.clone());
            group.clear();
        }
    }

    //Adiciona tambem se ouver valores restantes que não conseguiram formar um subgrupo
    // if !group.is_empty() {
    //     subgroups.push(group);
    // }

    subgroups
}

pub fn vec_u8_to_decimal(subgroups: &[Vec<u8>]) -> Vec<u32> {
    let mut vec_decimals = Vec::with_capacity(subgroups.len());

    for binary_vec in subgroups {
        let mut decimal: u32 = 0;
        for (i, &bit) in binary_vec.iter().rev().enumerate() {
            if bit == 1 {
                decimal += 1 << i;
            }
        }
        vec_decimals.push(decimal)
    }
    vec_decimals
}

/**
* o 'a serve para denotar explicitamente que o tempo de vida de a e b será o mesmo
*/
pub fn select_chromosome<'a>(chromosome_a: &'a [u8], chromosome_b: &'a [u8]) -> &'a [u8] {
    if fitness(chromosome_a).1 > fitness(chromosome_b).1 {
        chromosome_a
    } else {
        chromosome_b
    }
}

pub fn fitness(chromosome: &[u8]) -> (Vec<u8>, u64) {
    todo!("Função de fitness");
}

fn crossover(chromosome_a: &[u8], chromosome_b: &[u8]) -> Vec<u8> {
    let mut chrom_a = chromosome_a.to_vec();
    let mut chrom_b = chromosome_b.to_vec();

    if chrom_a.len() % 2 != 0 {
        chrom_a.insert(0, 0);
    }

    if chrom_b.len() % 2 != 0 {
        chrom_b.insert(0, 0);
    }

    match chrom_a.len() > chrom_b.len() {
        true => {
            let diff: usize = chrom_a.len() - chrom_b.len();
            chrom_b = [vec![0; diff], chrom_b].concat();
        }
        false => match chrom_b.len() > chrom_a.len() {
            true => {
                let diff = chrom_b.len() - chrom_a.len();
                chrom_a = [vec![0; diff], chrom_a].concat();
            }
            false => (),
        },
    }

    let mid = chrom_a.len() / 2;
    let first_half = &chrom_a[..mid];
    let second_half = &chrom_b[mid..];

    let mut new_chromosome = Vec::with_capacity(chrom_a.len());
    new_chromosome.extend(first_half);
    new_chromosome.extend(second_half);

    new_chromosome
}

pub fn mutation(chromosome: &[u8], mutation_tax: f64) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut genes: Vec<u8> = chromosome.to_vec();

    (0..genes.len()).for_each(|i| {
        let r: f64 = rng.gen_range(0.0..=1.0);

        if r <= mutation_tax {
            if genes[i] == 0 {
                genes[i] = 1;
            } else {
                genes[i] = 0;
            }
        }
    });

    genes
}

pub fn generate_new_pop(init_pop: &[Vec<u8>], mutation_tax: f64) -> Vec<Vec<u8>> {
    let mut new_pop: Vec<Vec<u8>> = Vec::new();
    let mut rng = rand::thread_rng();
    while new_pop.len() < init_pop.len() {
        let chromosome_a = &init_pop[rng.gen_range(0..init_pop.len())];
        let chromosome_b = &init_pop[rng.gen_range(0..init_pop.len())];
        let best_chromosome_a = select_chromosome(chromosome_a, chromosome_b).to_vec();

        let chromosome_a = &init_pop[rng.gen_range(0..init_pop.len())];
        let chromosome_b = &init_pop[rng.gen_range(0..init_pop.len())];
        let best_chromosome_b = select_chromosome(chromosome_a, chromosome_b).to_vec();

        let new_chromosome = crossover(&best_chromosome_a, &best_chromosome_b);
        let child = mutation(&new_chromosome, mutation_tax);

        new_pop.push(child);
    }

    new_pop
}
