use std::sync::Arc;

use my_http_server::{HttpFailResult, HttpOkResult};
use rust_extensions::{StopWatch, StringBuilder};

use crate::app::logs::LogItem;

pub fn compile_result(
    title: &str,
    logs: Vec<Arc<LogItem>>,
    mut sw: StopWatch,
) -> Result<HttpOkResult, HttpFailResult> {
    let mut sb = StringBuilder::new();

    sb.append_line(
        "<a class='btn btn-outline-secondary btn-sm' href='/logs'>Show All Log records</a>",
    );

    sb.append_line(
        "<a class='btn btn-outline-secondary btn-sm' href='/logs/table'>Show Log records by table</a>",
    );

    sb.append_line(
        "<a class='btn btn-outline-secondary btn-sm' href='/logs/process'>Show Log records by process</a>",
    );

    sb.append_line("<hr/>");

    for log_item in &logs {
        let line = format!(
            "<b style='background:{color}; color:white;'>{level:?}:</b> {dt}</br>",
            color = get_log_level_color(&log_item.as_ref()),
            dt = log_item.date.to_rfc3339(),
            level = log_item.level
        );
        sb.append_line(&line);

        if let Some(table_name) = &log_item.table {
            let line = format!(
                "<b>Table:</b> <a href='/logs/table/{table_name}'>{table_name}</a></br>",
                table_name = table_name
            );
            sb.append_line(line.as_str());
        }

        let line = format!(
            "<b>Process:</b> <a href='/logs/process/{process:?}'>{process:?}</a></br>",
            process = log_item.process
        );
        sb.append_line(line.as_str());

        let line = format!("<b>Process Name:</b> {}</br>", log_item.process_name);
        sb.append_line(line.as_str());

        let line = format!("<b>Msg:</b> {}</br>", log_item.message.as_str());
        sb.append_line(line.as_str());

        if let Some(err_ctx) = &log_item.ctx {
            let line = format!("<b>ErrCTX:</b> {:?}</br>", err_ctx);
            sb.append_line(line.as_str());
        }

        sb.append_line("<hr/>");
    }

    sw.pause();

    let line = format!("Rendered in {:?}", sw.duration());
    sb.append_line(line.as_str());

    super::super::as_html::build(title, sb.to_string_utf8().unwrap().as_str()).into_ok_result(true)
}

fn get_log_level_color(item: &LogItem) -> &str {
    match &item.level {
        crate::app::logs::LogLevel::Info => "green",
        crate::app::logs::LogLevel::Error => "orange",
        crate::app::logs::LogLevel::FatalError => "red",
    }
}
