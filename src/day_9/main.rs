fn main() {
    let input = "2333133121414131402";
    let blocks = get_blocks(input);
    println!("Blocks: {:?}", blocks);
    let compressed = compress_blocks(blocks);
    println!("Compressed Blocks: {:?}", compressed);
    println!("Checksum: {}", compute_checksum(compressed));
}

fn get_blocks(diskmap: &str) -> Vec<String> {
    let mut i = 0;
    let mut blocks: Vec<String> = Vec::new();
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
            blocks.push(current_id.to_string());
        }
        for _ in 0..free_space {
            blocks.push(".".to_string());
        }

        i += slice_len;
        current_id += 1;
    }

    blocks
}

fn compute_checksum(compressed_blocks: Vec<String>) -> i64 {
    let mut checksum: i64 = 0;
    for (i, n) in compressed_blocks.iter().enumerate() {
        if n != "." {
            let val = n.parse::<i64>().unwrap();
            checksum += val * i as i64;
        }
    }
    checksum
}

fn compress_blocks(blocks: Vec<String>) -> Vec<String> {
    let mut compressed = blocks.clone();
    let mut next_free_space = blocks.iter().position(|x| x == ".").unwrap();

    for i in 1..blocks.len() {
        let idx = blocks.len() - i;
        if blocks[blocks.len() - i].parse::<i32>().is_ok() {
            compressed.swap(idx, next_free_space);
            next_free_space = compressed.iter().position(|x| x == ".").unwrap();
        }
        if next_free_space > idx - 1 {
            break;
        }
    }

    // TODO: We need to terminate once we are fully compressed

    compressed
}

#[cfg(test)]
mod tests {
    use crate::{compress_blocks, compute_checksum, get_blocks};

    #[test]
    fn generate_blocks() {
        let a = "12345";
        assert_eq!(get_blocks(a).concat(), "0..111....22222");
    }

    #[test]
    fn generate_blocks_medium() {
        let a = "2333133121414131402";
        assert_eq!(
            get_blocks(a).concat(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_compression() {
        let blocks = get_blocks("12345");
        let compressed = compress_blocks(blocks);
        assert_eq!(compressed.join(""), "022111222......");
    }

    #[test]
    fn test_compression_large() {
        let blocks = get_blocks("2333133121414131402");
        let compressed = compress_blocks(blocks);
        assert_eq!(
            compressed.join(""),
            "0099811188827773336446555566.............."
        );
    }

    #[test]
    fn test_checksum() {
        let blocks = get_blocks("2333133121414131402");
        let compressed = compress_blocks(blocks);
        let checksum = compute_checksum(compressed);
        assert_eq!(checksum, 1928);
    }
}
