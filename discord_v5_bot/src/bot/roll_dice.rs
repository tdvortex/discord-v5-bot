use rand::prelude::*;

pub fn roll_dice(black_num: u8, red_num: u8) -> std::string::String {
    let mut rng = thread_rng();
    let mut black_digits: Vec<u8> = Vec::new();
    for _ in 0..black_num {
        black_digits.push(rng.gen_range(1..=10));
    }
    black_digits.sort();
    black_digits.reverse();

    let mut successes: u8 = 0;
    let mut crits: u8 = 0;
    let mut output = "".to_owned();

    for b in black_digits {
        if b == 10 {
            successes += 1;
            crits += 1;
            output.push_str("<:Stars:727127987043958817> ");
        } else if b > 5 {
            successes += 1;
            output.push_str("<:AnkhBlack:727127902490984450> ");
        } else {
            output.push_str("<:BlankBlack:727127939413704704> ");
        }
    }

    let mut red_digits: Vec<u8> = Vec::new();
    for _ in 0..red_num {
        red_digits.push(rng.gen_range(1..=10));
    }
    red_digits.sort();
    red_digits.reverse();

    let mut has_skull = false;
    let mut has_fangs = false;

    for r in red_digits {
        if r == 10 {
            successes += 1;
            crits += 1;
            has_fangs = true;
            output.push_str("<:Fangs:727127963639742506> ");
        } else if r > 5 {
            successes += 1;
            output.push_str("<:AnkhRed:727127924721057813> ");
        } else if r == 1 {
            has_skull = true;
            output.push_str("<:Skull:727127976352940132> ");
        } else {
            output.push_str("<:BlankRed:727127952243949610> ");
        }
    }

    let mut is_messy = false;

    while crits >= 2 {
        successes += 2;
        crits -= 2;
        if has_fangs {
            is_messy = true;
        }
    }

    let successes_count_str = format!(", {} successes", successes);
    output.push_str(&successes_count_str);

    if is_messy {
        output.push_str(", **messy**");
    }

    if has_skull {
        output.push_str(", **bestial**");
    }

    output
}
