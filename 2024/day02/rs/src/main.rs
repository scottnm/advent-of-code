use input_helpers;
use std::process::ExitCode;

type ReportData = Vec<isize>;

fn read_report_data_from_input(filename: &str) -> Result<Vec<ReportData>, String> {
    let lines = input_helpers::read_lines(filename);
    let mut reports: Vec<ReportData> = Vec::new();
    for line in lines {
        let values: Vec<&str> = line.split_ascii_whitespace().collect();
        if values.len() < 2 {
            return Err(format!(
                "Invalid number of values on line. Expected at least 2 found {}. line={}",
                values.len(),
                line
            ));
        }

        let mut report: ReportData = ReportData::new();
        for v in values {
            match v.parse() {
                Ok(value) => report.push(value),
                Err(e) => return Err(format!("Invalid value: {}! '{}' line={}", e, v, line)),
            }
        }

        reports.push(report);
    }

    Ok(reports)
}

fn is_data_pair_safe(d1: isize, d2: isize, data_trend_increasing: Option<bool>) -> bool {
    let diff = d2 - d1;
    let abs_diff = diff.abs();
    if abs_diff > 3 || 1 > abs_diff {
        return false;
    }

    let diff_increasing = diff > 0;
    assert!(diff != 0);

    data_trend_increasing.is_none() || data_trend_increasing == Some(diff_increasing)
}

fn is_report_data_safe(report_data: &[isize]) -> bool {
    let mut all_data_increasing: Option<bool> = None;

    for i in 1..report_data.len() {
        if is_data_pair_safe(report_data[i - 1], report_data[i], all_data_increasing) {
            all_data_increasing = Some(report_data[i] > report_data[i - 1]);
        } else {
            return false;
        }
    }

    true
}

fn is_dampened_report_data_safe(report_data: &[isize]) -> bool {
    let mut has_skipped_report = false;
    let mut all_data_increasing: Option<bool> = None;

    let mut report_safe = true;
    let mut i = 1;
    while i < report_data.len() {
        // if we're still safe keep moving
        if is_data_pair_safe(report_data[i - 1], report_data[i], all_data_increasing) {
            all_data_increasing = Some(report_data[i] > report_data[i - 1]);
            i += 1;
        }
        // if this data point would make us unsafe, try to dampen either the data point at position i or i - 1
        else if !has_skipped_report {
            if i == report_data.len() - 1 {
                // we can just remove the last data point and be safe
                i += 2;
                has_skipped_report = true;
            } else if is_data_pair_safe(report_data[i - 1], report_data[i + 1], all_data_increasing)
            {
                // we can just skip the ith data point and be safe
                all_data_increasing = Some(report_data[i + 1] > report_data[i - 1]);
                i += 2;
                has_skipped_report = true;
            } else if i == 2 && is_data_pair_safe(report_data[i - 2], report_data[i], None) {
                // skip the 1st data point and be safe.
                // N.B. if we're skipping the first data point we HAVE to re-calculate the all_data_increasing value
                all_data_increasing = Some(report_data[i] > report_data[i - 2]);
                i += 1;
                has_skipped_report = true;
            } else if i > 2
                && is_data_pair_safe(report_data[i - 2], report_data[i], all_data_increasing)
            {
                // skip the i-1th data point and be safe.
                all_data_increasing = Some(report_data[i] > report_data[i - 2]);
                i += 1;
                has_skipped_report = true;
            } else {
                // even after skipping all of our candidate data points we're still not safe so just mark the
                // report as not safe and move on
                report_safe = false;
                break;
            }
        } else {
            // N.B. we've already skipped one report which was unsafe. Another unsafe report means this report can't be dampened to safety.
            report_safe = false;
            break;
        }
    }

    // N.B. if the report isn't safe after attempting to dampen any of the values AFTER the first value,
    // we might still be a safe report if we try dampening the first value. Special case this check to simplify loop logic above
    if !report_safe {
        if report_data.len() >= 3 {
            report_safe = is_report_data_safe(&report_data[1..])
        }
    }

    report_safe
}

fn is_dampened_report_data_safe_brute(report_data: &[isize]) -> bool {
    // Check if our report data is safe without dampening
    if !is_report_data_safe(report_data) {
        let mut tmpbuf = Vec::with_capacity(report_data.len());

        // Brute force. Iterate over the report data and try to check if removing each
        // data point in isolation makes us safe.
        for i in 0..report_data.len() {
            // Copy dampened report data to a tmp buffer to make reusing the is_report_data_safe
            // helper easier
            tmpbuf.clear();
            for j in 0..report_data.len() {
                if j != i {
                    tmpbuf.push(report_data[j]);
                }
            }

            if is_report_data_safe(&tmpbuf) {
                return true;
            }
        }

        return false;
    }

    true
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Not enough args");
        return ExitCode::FAILURE;
    }

    let filename: &str = &args[0];

    let parse_result = read_report_data_from_input(filename);
    let reports = match parse_result {
        Ok(parsed_reports) => parsed_reports,
        Err(e) => {
            println!("Invalid input! {}", e);
            return ExitCode::FAILURE;
        }
    };

    let raw_start_time = std::time::Instant::now();
    let safe_report_count: usize = reports.iter().filter(|r| is_report_data_safe(&r)).count();
    let unsafe_report_count = reports.len() - safe_report_count;
    println!("RAW: ({:0.06}s)", raw_start_time.elapsed().as_secs_f64());
    println!("--------------------------");
    println!("Safe report count: {}", safe_report_count);
    println!("Unsafe report count: {}", unsafe_report_count);
    println!("");

    let dampened_start_time = std::time::Instant::now();
    let adj_safe_report_count: usize = reports
        .iter()
        .filter(|r| is_dampened_report_data_safe(&r))
        .count();
    let adj_unsafe_report_count = reports.len() - adj_safe_report_count;
    println!(
        "DAMPENED: ({:0.06}s)",
        dampened_start_time.elapsed().as_secs_f64()
    );
    println!("--------------------------");
    println!("Safe report count: {}", adj_safe_report_count);
    println!("Unsafe report count: {}", adj_unsafe_report_count);
    println!("");

    let dampened_brute_start_time = std::time::Instant::now();
    let adj_safe_report_count: usize = reports
        .iter()
        .filter(|r| is_dampened_report_data_safe_brute(&r))
        .count();
    let adj_unsafe_report_count = reports.len() - adj_safe_report_count;
    println!(
        "DAMPENED BRUTE: ({:0.06}s)",
        dampened_brute_start_time.elapsed().as_secs_f64()
    );
    println!("--------------------------");
    println!("Safe report count: {}", adj_safe_report_count);
    println!("Unsafe report count: {}", adj_unsafe_report_count);

    for (i, report) in reports.iter().enumerate() {
        let safe_res = is_dampened_report_data_safe(report);
        let safe_res_brute = is_dampened_report_data_safe_brute(report);
        if safe_res != safe_res_brute {
            println!(
                "Report {:02} safety results differed! Expected {}. Got {}.",
                i, safe_res_brute, safe_res
            );
            println!("    report = {:#?}", report);
        }
    }

    return ExitCode::SUCCESS;
}
