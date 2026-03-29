//! Terminal histogram for sampling results

use intrico_core::SamplingResult;

pub fn plot_histogram(result: &SamplingResult) {
    if result.counts.is_empty() {
        println!("\n No measurement data to plot.\n");
        return;
    }

    let width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80);

    // Sort entries by bitstring
    let mut entries: Vec<(&String, &usize)> = result.counts.iter().collect();
    entries.sort_by_key(|(k, _)| (*k).clone());

    let max_count = *entries.iter().map(|(_, c)| *c).max().unwrap();
    let total = result.shots;

    // Calculate column widths
    let max_label_len = entries.iter().map(|(k, _)| k.len()).max().unwrap();
    let label_col = max_label_len + 4; // " |XX⟩  "
    let max_count_digits = max_count.to_string().len();
    let stats_col = max_count_digits + 11; // "  NNN (XX.XX%)"
    let bar_max = ((width.saturating_sub(label_col + stats_col + 2)) as f64 * 0.4) as usize;

    // Header
    let header = format!("Histogram ({} shots)", total);
    println!("\n {}", header);
    println!(" {}", "\u{2500}".repeat(header.len()));

    for &(bitstring, &count) in &entries {
        let pct = count as f64 / total as f64 * 100.0;
        let bar_len = if bar_max > 0 && max_count > 0 {
            (count as f64 / max_count as f64 * bar_max as f64).round() as usize
        } else {
            0
        };

        let bar = "\u{2588}".repeat(bar_len);

        println!(
            " |{}⟩  {:<bw$}  {:>cw$} ({:>5.2}%)",
            bitstring,
            bar,
            count,
            pct,
            bw = bar_max,
            cw = max_count_digits,
        );
    }
    println!();
}
