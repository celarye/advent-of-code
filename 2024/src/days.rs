mod _1;
mod _2;
mod _3;
mod _4;
mod _5;
mod _6;

pub fn results() -> Vec<Vec<String>> {
    let _1_results;
    match _1::init() {
        Ok(_1) => _1_results = _1.results(),
        Err(()) => _1_results = vec!["Error".to_string(), "Error".to_string()],
    }

    let _2_results;
    match _2::init() {
        Ok(_2) => _2_results = _2.results(),
        Err(()) => _2_results = vec!["Error".to_string(), "Error".to_string()],
    }

    let _3_results;
    match _3::init() {
        Ok(_3) => _3_results = _3.results(),
        Err(()) => _3_results = vec!["Error".to_string(), "Error".to_string()],
    }

    let _4_results;
    match _4::init() {
        Ok(_4) => _4_results = _4.results(),
        Err(()) => _4_results = vec!["Error".to_string(), "Error".to_string()],
    }

    let _5_results;
    match _5::init() {
        Ok(_5) => _5_results = _5.results(),
        Err(()) => _5_results = vec!["Error".to_string(), "Error".to_string()],
    }

    let _6_results;
    match _6::init() {
        Ok(_6) => _6_results = _6.results(),
        Err(()) => _6_results = vec!["Error".to_string(), "Error".to_string()],
    }

    vec![
        _1_results, _2_results, _3_results, _4_results, _5_results, _6_results,
    ]
}
