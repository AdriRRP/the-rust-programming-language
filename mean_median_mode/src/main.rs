use std::collections::HashMap;

struct Metrics {
    mean: Option<f64>,
    median: Option<f64>,
    mode: Vec<f64>,
}

fn main() {
    let mut v = Vec::new();

    let metrics = get_metrics(&mut v);

    println!(
        "Vec: {:?}\nMean: {:?} Median: {:?} Mode: {:?}",
        v, metrics.mean, metrics.median, metrics.mode
    );

    let mut v = vec![1.0];

    let metrics = get_metrics(&mut v);

    println!(
        "Vec: {:?}\nMean: {:?} Median: {:?} Mode: {:?}",
        v, metrics.mean, metrics.median, metrics.mode
    );

    let mut v = vec![1.0, 2.0];

    let metrics = get_metrics(&mut v);

    println!(
        "Vec: {:?}\nMean: {:?} Median: {:?} Mode: {:?}",
        v, metrics.mean, metrics.median, metrics.mode
    );

    let mut v = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let metrics = get_metrics(&mut v);

    println!(
        "Vec: {:?}\nMean: {:?} Median: {:?} Mode: {:?}",
        v, metrics.mean, metrics.median, metrics.mode
    );

    let mut v = vec![1.0, 2.0, 3.0, 3.0, 4.0, 5.0];

    let metrics = get_metrics(&mut v);

    println!(
        "Vec: {:?}\nMean: {:?} Median: {:?} Mode: {:?}",
        v, metrics.mean, metrics.median, metrics.mode
    );
}

fn get_metrics(v: &mut Vec<f64>) -> Metrics {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Metrics {
        mean: mean(&v),
        median: median(&v),
        mode: mode(&v),
    }
}

fn mean(v: &Vec<f64>) -> Option<f64> {
    if v.len() == 0 {
        None
    } else {
        let mut m: f64 = 0.0;

        for e in v {
            m += e;
        }

        Some(m / v.len() as f64)
    }
}

fn median(v: &Vec<f64>) -> Option<f64> {
    let n = v.len();

    if v.len() == 0 {
        None
    } else if n % 2 == 0 {
        let x1 = v[(n / 2) - 1];
        let x2 = v[n / 2];

        Some((x1 + x2) / 2.0)
    } else {
        Some(v[n / 2])
    }
}

fn mode(v: &Vec<f64>) -> Vec<f64> {
    if v.len() == 0 {
        Vec::new()
    } else {
        let mut map = HashMap::new();

        let mut n_max = 0;

        for n in v {
            let count = map.entry(n.to_string()).or_insert(0);

            *count += 1;

            if *count > n_max {
                n_max = *count;
            }
        }

        let mut result = Vec::new();

        for (key, value) in &map {
            if *value == n_max {
                result.push(key.parse().unwrap());
            }
        }

        result
    }
}
