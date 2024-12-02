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

fn is_report_data_safe(report_data: &[isize]) -> bool {
    // N.B. Should have already been validated when input was parsed.
    // FIXME: Maybe there's a more clever way to require the caller conform to this without making the function handle a potential error case.
    // FIXME: maybe I don't even care here
    // assert!(report_data.len() >= 2);

    let mut all_data_increasing: Option<bool> = None;

    for i in 1..report_data.len() {
        let diff = report_data[i] - report_data[i - 1];
        let abs_diff = diff.abs();
        if abs_diff > 3 || 1 > abs_diff {
            return false;
        }
        
        let diff_increasing = diff > 0;
        assert!(diff != 0);

        if let Some(all_data_increasing) = all_data_increasing {
            if all_data_increasing != diff_increasing {
                return false;
            }
        } else {
            all_data_increasing = Some(diff_increasing);
        }
    }

    true
}

// FIXME: refactor to share this helper between two impls
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

fn is_dampened_report_data_safe(report_data: &[isize]) -> bool {
    // N.B. Should have already been validated when input was parsed.
    // FIXME: Maybe there's a more clever way to require the caller conform to this without making the function handle a potential error case.
    assert!(report_data.len() >= 2);

    let mut has_skipped_report = false;
    let mut all_data_increasing: Option<bool> = None;

    let mut report_safe = true;
    let mut i = 1;
    while i < report_data.len() {
        if is_data_pair_safe(report_data[i - 1], report_data[i], all_data_increasing) {
            all_data_increasing = Some(report_data[i] > report_data[i - 1]);
            i += 1;
        } else if !has_skipped_report {
            if i == report_data.len() - 1 {
                // we can just remove the last data point and be safe
                i += 2;
                has_skipped_report = true;
            } else if is_data_pair_safe(report_data[i - 1], report_data[i + 1], all_data_increasing) {
                // we can just skip the ith report and be safe
                all_data_increasing = Some(report_data[i + 1] > report_data[i - 1]);
                i += 2;
                has_skipped_report = true;
            } else if i > 2 && is_data_pair_safe(report_data[i - 2], report_data[i], all_data_increasing) {
                all_data_increasing = Some(report_data[i] > report_data[i - 2]);
                i += 1;
                has_skipped_report = true;
            } else if i == 2 && is_data_pair_safe(report_data[i - 2], report_data[i], None) {
                all_data_increasing = Some(report_data[i] > report_data[i - 2]);
                i += 1;
                has_skipped_report = true;
            } else {
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
            report_safe = is_report_data_safe(&report_data[1..]) || is_report_data_safe(&report_data[..report_data.len()-1]);
        }
    }

    report_safe
}

fn is_dampened_report_data_safe_brute(report_data: &[isize]) -> bool {
    // N.B. Should have already been validated when input was parsed.
    // FIXME: Maybe there's a more clever way to require the caller conform to this without making the function handle a potential error case.
    // assert!(report_data.len() >= 2);

    if !is_report_data_safe(report_data) {
        let mut tmpbuf = Vec::with_capacity(report_data.len());

        for i in 0..report_data.len() {
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

    let safe_report_count: usize = reports.iter().filter(|r| is_report_data_safe(&r)).count();
    println!("RAW:");
    println!("--------------------------");
    println!("Safe report count: {}", safe_report_count);
    println!("Unsafe report count: {}", reports.len() - safe_report_count);
    println!("");

    let adj_safe_report_count: usize = reports.iter().filter(|r| is_dampened_report_data_safe(&r)).count();
    println!("DAMPENED:");
    println!("--------------------------");
    println!("Safe report count: {}", adj_safe_report_count);
    println!("Unsafe report count: {}", reports.len() - adj_safe_report_count);
    println!("");

    let adj_safe_report_count: usize = reports.iter().filter(|r| is_dampened_report_data_safe_brute(&r)).count();
    println!("DAMPENED BRUTE:");
    println!("--------------------------");
    println!("Safe report count: {}", adj_safe_report_count);
    println!("Unsafe report count: {}", reports.len() - adj_safe_report_count);

    for (i,report) in reports.iter().enumerate() {
        let safe_res = is_dampened_report_data_safe(report);
        let safe_res_brute = is_dampened_report_data_safe_brute(report);
        if safe_res != safe_res_brute {
            println!("Report {:02} safety results differed! Expected {}. Got {}.", i, safe_res_brute, safe_res);
            println!("    report = {:#?}", report);
        }
    }

    /*
    let similarity_score = calculate_similarity_score(&input_pairs);
    println!("Similarity score: {}", similarity_score);
    */

    return ExitCode::SUCCESS;
}
