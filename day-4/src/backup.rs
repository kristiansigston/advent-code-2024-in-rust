let start_unoptimized = Instant::now();
let unoptimized_count = check_all_directions(&word_search, &mut total_words);
let duration_unoptimized = start_unoptimized.elapsed();

let start_optimized = Instant::now();
let optimized_count = check_all_directions_optimised(&word_search, &mut total_words);
let duration_optimized = start_optimized.elapsed();