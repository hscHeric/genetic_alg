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
