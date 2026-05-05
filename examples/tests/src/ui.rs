use crate::state::{PAGE_INDEX, RESULTS};

use gooseboy::framebuffer::{get_framebuffer_height, get_framebuffer_width};
use gooseboy::input::is_key_just_pressed;
use gooseboy::keys::{KEY_A, KEY_D, KEY_LEFT, KEY_RIGHT};
use gooseboy::text::{draw_text_formatted, get_formatted_text_width};
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

#[allow(clippy::significant_drop_tightening)]
pub fn render() {
    clear_framebuffer(Color::BLACK);

    let results_guard = RESULTS.lock().unwrap();
    let results = results_guard.as_slice();

    let ok_count = results.iter().filter(|f| f.status).count();
    let fail_count = results.iter().filter(|f| !f.status).count();
    let summary = format!("{ok_count} [green]OK[white] {fail_count} [red]FAIL");

    let fbw = get_framebuffer_width();
    draw_text_formatted(
        (fbw / 2).saturating_sub(get_formatted_text_width(&summary) / 2),
        0,
        summary,
        Color::WHITE,
    );

    let fbh = get_framebuffer_height();
    let header_h: usize = 16;
    let footer_h: usize = 8;
    let available_pixels = if fbh > header_h + footer_h {
        fbh - header_h - footer_h
    } else {
        0
    };
    let mut lines_per_page = available_pixels / 8;
    if lines_per_page == 0 {
        lines_per_page = 1;
    }

    let total_results = results.len();
    let page_count = if total_results == 0 {
        1usize
    } else {
        total_results.div_ceil(lines_per_page)
    };

    {
        let mut page = PAGE_INDEX.lock().unwrap();
        if is_key_just_pressed(KEY_RIGHT) || is_key_just_pressed(KEY_D) {
            *page = (*page + 1) % page_count;
        }
        if is_key_just_pressed(KEY_LEFT) || is_key_just_pressed(KEY_A) {
            if *page == 0 {
                *page = page_count.saturating_sub(1);
            } else {
                *page -= 1;
            }
        }
    }

    let page = *PAGE_INDEX.lock().unwrap();

    let start_idx = page.saturating_mul(lines_per_page);
    for row in 0..lines_per_page {
        let idx = start_idx + row;
        if idx >= total_results {
            break;
        }
        let result = &results[idx];
        let y = header_h + (row * 8);

        draw_text_formatted(
            0,
            y,
            format!(
                "{} [{}]{}",
                result.name.clone(),
                if result.status { "green" } else { "red" },
                if result.status { "OK" } else { "FAIL" }
            ),
            Color::WHITE,
        );
    }

    let page_text = format!("Page {}/{}", page + 1, page_count);
    let px = (fbw / 2).saturating_sub(get_formatted_text_width(&page_text) / 2);
    let py = fbh.saturating_sub(footer_h);
    draw_text_formatted(px, py, page_text, Color::WHITE);
}
