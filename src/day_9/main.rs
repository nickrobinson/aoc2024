fn main() {
    let input = "12345";
    let blocks = get_blocks(input);
    println!("Blocks: {}", blocks);
}

fn get_blocks(diskmap: &str) -> String {
    let mut i = 0;
    let mut blocks = String::new();
    let mut current_id = 0;
    while i < diskmap.len() {
        // Determine the length of the slice: 2 if there are at least 2 chars left, otherwise 1.
        let slice_len = if i + 2 <= diskmap.len() { 2 } else { 1 };
        let chunk = &diskmap[i..i + slice_len];
        let file_count = chunk.chars().next().unwrap().to_digit(10);
        let free_space = match chunk.chars().nth(1) {
            None => 0,
            Some(i) => i.to_digit(10).unwrap() as i32,
        };

        for _ in 0..file_count.unwrap() {
            blocks.push_str(&current_id.to_string());
        }
        for _ in 0..free_space {
            blocks.push('.');
        }

        i += slice_len;
        current_id += 1;
    }

    blocks
}

#[cfg(test)]
mod tests {
    use crate::get_blocks;

    #[test]
    fn generate_blocks() {
        let a = "12345";
        assert_eq!(get_blocks(a), "0..111....22222");
    }

    #[test]
    fn generate_blocks_medium() {
        let a = "2333133121414131402";
        assert_eq!(get_blocks(a), "00...111...2...333.44.5555.6666.777.888899");
    }
}
