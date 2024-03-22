use std::fs::{self, File};
use std::io::{self, Read};
use std::mem;
use std::time::{Duration, Instant};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use stateful_filter::{FilterBuilder, filter::first_order::FirstOrderIirFilter, filter::Filter};
use plotly::{Plot, Scatter};


fn read_file_to_vec(filename: &str) -> io::Result<Vec<u8>> {
    // Open the file in read-only mode
    let mut file = File::open(filename)?;

    // Get the metadata of the file to determine its size
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;

    // Create a buffer to store the file content
    let mut buffer = vec![0; file_size];

    // Read the content of the file into the buffer
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

fn to_u16_le(data: &[u8]) -> Option<u16> {
    if data.len() >= 2 {
        // Combine the first and second bytes into a single u16 in little-endian order
        let value = (data[0] as u16) | ((data[1] as u16) << 8);
        Some(value)
    } else {
        None
    }
}

fn to_u64_le(data: &[u8]) -> Option<u64> {
    if data.len() >= 8 {
        // Combine the first 8 bytes into a single u64 in little-endian order
        let mut value: u64 = 0;
        for i in 0..8 {
            value |= (data[i] as u64) << (8 * i);
        }
        Some(value)
    } else {
        None
    }
}

fn to_f64(data: &[u8]) -> Option<f64> {
    if data.len() >= 8 {
        // Combine the first 8 bytes into a single u64
        let mut value: u64 = 0;
        for i in 0..8 {
            value |= (data[i] as u64) << (8 * i);
        }
        // Transmute the u64 into an f64
        Some(unsafe { mem::transmute::<u64, f64>(value) })
    } else {
        None
    }
}

fn standard_deviation(data: &[f64]) -> Option<f64> {
    let n = data.len();
    if n <= 1 {
        return None; // Standard deviation is undefined for one or zero elements
    }
    // Calculate the mean
    let mean = data.par_iter().sum::<f64>() / (n as f64);
    // Calculate the sum of squares of differences from the mean
    let sum_squared_diffs = data.par_iter().map(|&x| (x - mean).powi(2)).sum::<f64>();
    // Calculate the variance
    let variance = sum_squared_diffs / ((n - 1) as f64); // using Bessel's correction
    // Standard deviation is the square root of variance
    Some(variance.sqrt())
}

fn analyze(fname: &str) {
    println!("/////////////// ANALYZING FILE {:?} ///////////////", fname);
    let Ok(res) = read_file_to_vec(fname) else {
        return eprintln!("File not found")
    };
    let events_num = to_u64_le(&res[0..8]).unwrap();
    let events_num_idx = events_num as usize * 8;
    let centroids = res.clone()[8..(events_num_idx + 8)].to_vec();
    let centroids: Vec<_> = centroids.chunks_exact(8)
    .map(|c| to_u64_le(c))
    .flatten()
    .collect();
    for i in 0..15 {
        println!(" {:?}", &centroids[i]);
    }
    let f = 8 + events_num_idx;
    let t = f + 8;
    let dt = to_f64(&res[f..t]).unwrap();
    let f = t;
    let t = f + 8;
    let resolution = to_f64(&res[f..t]).unwrap();
    println!("Sampling rate: {:?}", 1.0 / dt);
    let f = t;
    let t = f + 8;
    let data_points = to_u64_le(&res[f..t]).unwrap();
    println!("Number of points: {:?}", data_points);
    let f = t;
    let t = f + data_points as usize * 2;
    let data = res.clone()[f..t].to_vec();
    let data: Vec<_> = data.chunks(2)
    .map(|f| to_u16_le(f))
    .flatten()
    .map(|f| f as f64 * resolution)
    .collect();
    let low_pass_low_a = vec![1.0, -0.9994];
    let low_pass_low_b = vec![0.0003, 0.0003];
    let low_pass_high_a = vec![1.0, -0.5095];
    let low_pass_high_b = vec![0.2452, 0.2452];
    let mut low_f = FilterBuilder::from(&low_pass_low_a, &low_pass_low_b);
    let mut high_f = FilterBuilder::from(&low_pass_high_a, &low_pass_high_b);
    // let mut low_f = FirstOrderIirFilter::new(low_pass_low_a[1], (low_pass_low_b[0], low_pass_low_b[1]));
    // let mut high_f = FirstOrderIirFilter::new(low_pass_high_a[1], (low_pass_high_b[0], low_pass_high_b[1]));
    let mut time_not_to_count = Duration::new(0, 0);
    let mut shrinkable_centroids:Vec<_> = centroids.clone();
    let mut true_positives = 0;
    let mut false_positives = 0;
    let mut false_negatives = 0;
    let start = Instant::now();
    // let ms_100 = 10000;
    let ms_100 = 100000;
    // let mut plot = Plot::new();
    for i in 0.. {
        let chunk_begin = i * ms_100;
        if chunk_begin > data.len() {
            break;
        }
        let chunk_end = chunk_begin + ms_100;
        let chunk_end = if chunk_end > data.len() {data.len()} else {chunk_end};
        let c = &data[chunk_begin..chunk_end];
        low_f.init(c[0]);
        let baseline = low_f.filt(&c);
        let sig_no_baseline: Vec<_> = c.into_iter().zip(&baseline).map(|(s,b)| s-b).collect();
        // let trace = Scatter::new((0..c.len()).map(|f| f as f64).collect(), sig_no_baseline.to_vec());
        // plot.add_trace(trace);
        let events_track = high_f.filt(&sig_no_baseline);
        // let trace = Scatter::new((0..c.len()).map(|f| f as f64).collect(), sig_no_baseline.to_vec());
        // plot.add_trace(trace);
        // let centroids_to_plot: Vec<_> = (0..c.len()).map(|f| if centroids.contains(&(f as u64)){200.0} else {0.0}).collect();
        // let trace = Scatter::new((0..c.len()).map(|f| f as f64).collect(), centroids_to_plot);
        // plot.add_trace(trace);
        let std_dev = standard_deviation(&events_track).unwrap();
        let mut event_idxes: Vec<_> = events_track.clone().into_iter().enumerate()
        .filter_map(|(idx, d)| if d < -3.0 * std_dev {Some((idx + chunk_begin) as u64)} else {None})
        .collect();
        // let mut event_idxes2: Vec<_> = events_track.into_iter()
        // .filter_map(|d| if d < -3.0 * std_dev {Some(100.0)} else {Some(0.0)})
        // .collect();
        // let trace: Box<Scatter<f64, f64>> = Scatter::new((0..c.len()).map(|f| f as f64).collect(), event_idxes2);
        // plot.add_trace(trace);

        let new_start = Instant::now();
        let mut even_len = 0;
        let mut prev_value = if event_idxes.len() > 0 {
            let first_event = event_idxes[0];
            event_idxes.remove(0);
            first_event
        } else {
            0
        };
        let mut already_found_centroid = false;
        for e in event_idxes {
            if e == prev_value + 1 {
                if shrinkable_centroids.contains(&e) {
                    let index = shrinkable_centroids.iter().position(|&r| r == e).unwrap();
                    shrinkable_centroids.remove(index);
                    true_positives += 1;
                    already_found_centroid = true;
                    even_len += 1;
                }
            } else {
                if !already_found_centroid && even_len > 2{
                    false_positives += 1;
                }
                even_len = 0;
                already_found_centroid = false;
            }
            prev_value = e;
        }
        time_not_to_count += new_start.elapsed();
    }
    let end = start.elapsed() - time_not_to_count;
    println!("Time elapsed: {:?}", end);
    false_negatives += shrinkable_centroids.len();
    println!("Precision: {:?} || Recall: {:?}", (true_positives as f64) / ((true_positives + false_positives) as f64), (true_positives as f64) / ((true_positives + false_negatives) as f64));
    println!("True posistives: {:?}, Centroids: {:?}, false positives: {:?}", true_positives, centroids.len(), false_positives);
    println!("Approximated speed: {:?} MB/s", (data_points as f64) / 1e6 / (end.as_millis() as f64) * 1000.0);
    // plot.show(); 
}

fn main() {
    let directory_path = r"E:\synthetic";
    // Get absolute paths of files in the specified directory
    if let Ok(entries) = fs::read_dir(directory_path) {
        // Iterate over directory entries
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                analyze(path.to_str().unwrap());
            }
        }
    } else {
        eprintln!("Failed to read directory {}", directory_path);
    }
    // analyze(r"E:\synthetic\20nmAuNP_50%PEG+50mMKCl_-500mV_60nm_pore.abf_1.dat");
}