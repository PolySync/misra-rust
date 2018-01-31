#[forbid(clippy_pedantic)]
#[forbid(clippy)]
#[forbid(warnings)]

fn main() {
    'L1: for _ in 0..5 {
        'L2: for _ in 0..4 {
            if 2 == 3 {
                break 'L1;
            }
        }
    }

    'L3: for _ in 0..4 {
        if 2 == 3 {
            break 'L2;
            //~^ ERROR use of undeclared label `'L2`
        }
    }
}
