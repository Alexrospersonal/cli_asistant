use std::error::Error;
use crate::cli::command_enum::SummaryArgs;
use crate::commands::track::db::open_db;

pub async fn run(args: SummaryArgs) -> Result<(), Box<dyn Error>> {
    let connection = open_db()?;
    
    let mut sql_string = "
            SELECT COUNT(*), SUM(duration), AVG(duration)
            FROM sessions
            WHERE ended_at IS NOT NULL
        ".to_string();
    
    if args.today {
        sql_string.push_str("\nAND DATE(started_at) = DATE('now')");
    }
    
    let mut stmt = connection.prepare(
        sql_string.as_str()
    )?;
    
     let res = stmt.query_row([], |row| {
        let count: i64 = row.get(0)?;
        let sum: Option<f64> = row.get(1)?;
        let avg: Option<f64> = row.get(2)?;
        Ok((count, sum, avg))
    });
    
    let (count, sum, avg) = res?;
    
    if count == 0 {
        print!("No completed sessions yet");
        return  Ok(())
    }
    
    let sum = sum.unwrap_or_else( || 0.0) as i64;
    let avg = avg.unwrap_or_else(|| 0.0) as i64;

    let hour = sum / 3600;
    let minute = (sum % 3600) / 60;
    
    let avg_in_min = (avg % 3600) / 60;
    
    let summary  = if args.today {
        "📊 Summary for today:"
    } else {
        "📊 Summary:"
    };
    
    let format = format!(
        "{}\
        \n\tTotal sessions: {}\
        \n\tTotal time: {}h {}m\
        \n\tAverage session:{}m",
        summary, count, hour, minute, avg_in_min
    );
    println!("{}", format);
    
    Ok(())
}