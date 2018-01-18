fn main() {
    'L1: for _ in 0..5 {
        'L2: for _ in 0..4 {
            break 'L1
        }
    }
}
