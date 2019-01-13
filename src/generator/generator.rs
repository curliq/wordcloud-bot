use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};
use std::cmp;

pub fn generate_wordcloud_list(
    words: Vec<String>,
    expression_max_length: u16,
) -> Vec<(String, u32)> {
    let mut expressions_agg: Vec<(String, u32)> = Vec::new();
    let mut word_index: usize = 0;

    if expression_max_length as u32 > words.len() as u32 {
        panic!("Expression length must be smaller than sample size");
    }

    // Iterate through every individual word
    for word in words.iter() {
        // Get n expressions for each word, i.e. the word plus the n-1 following words,
        // where n is expression_max_length
        // e.g. ["hi", "what's", "up", "hi"] with expression_max_length=2 yields
        // [("hi", 2), ("hi what's", 1), ("what's", 1), ("what's up", 1), ("up", 1), ("up hi", 1)]
        println!("{}", word);
        for size in 0..cmp::min(expression_max_length as usize, words.len() - word_index) {
            let mut expression: String = word.clone();
            for i in 1..cmp::min(size as usize + 1, words.len() - word_index) {
                let index = word_index + (i as usize);
                if index <= words.len() - 1 {
                    expression.push_str(" ");
                    expression.push_str(words[index].as_str());
                }
            }
            let expressions_flat: Vec<&String> = expressions_agg
                .iter()
                .map(|tup| &tup.0)
                .collect::<Vec<&String>>();
            if expressions_flat.contains(&&expression) {
                // find it and increment count
                let ind = expressions_flat
                    .iter()
                    .position(|r| r == &&expression)
                    .unwrap();
                expressions_agg[ind].1 += 1
            } else {
                // add
                expressions_agg.push((expression.to_string(), 0u32));
            }
        }

        // track word index manually to avoid expensive index calculations
        word_index += 1;
    }

    println!("finished creating array of wordcloud");
    expressions_agg.sort_by(|a, b| b.1.cmp(&a.1));
    return expressions_agg;
}

pub fn generate_image(words: Vec<(String, u32)>) -> String {
    let font_data = include_bytes!("../res/manrope-regular.ttf");
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let colour = (200, 200, 200);

    let font_size_range = (30, 80);
    let words_displayed = 50;

    //    let freq_highest = &words[0].1;
    //    let freq_lowest = &words[cmp::min(words_displayed, words.len()) - 1].1;

    let mut image = DynamicImage::new_rgba8(1000, 1000).to_rgba();

    let mut last_word_top_padding: f32 = 100.0;
    for i in 0..words_displayed {
        if &words.len() < &(&i + 1) || &last_word_top_padding > &800.0 {
            break;
        }
        let word = &words[i];
        let text = &word.0;
        //        let freq = &word.1;

        let font_size: i32 = font_size_range.1 as i32 - (i * 10) as i32;
        let scale = cmp::max(font_size.clone(), font_size_range.0);
        let scale = Scale::uniform(scale as f32);

        let v_metrics = font.v_metrics(scale);

        // layout the glyphs in a line with 20 pixels padding

        let glyphs: Vec<_> = font
            .layout(
                &text[..],
                scale,
                point(100.0, last_word_top_padding + v_metrics.ascent),
            )
            .collect();
        last_word_top_padding += cmp::max(font_size, font_size_range.0) as f32;
        //        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        //        let glyphs_width = {
        //            let min_x = glyphs
        //                .first()
        //                .map(|g| g.pixel_bounding_box().unwrap().min.x)
        //                .unwrap();
        //            let max_x = glyphs
        //                .last()
        //                .map(|g| g.pixel_bounding_box().unwrap().max.x)
        //                .unwrap();
        //            (max_x - min_x) as u32
        //        };

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba {
                            data: [colour.0, colour.1, colour.2, (v * 255.0) as u8],
                        },
                    )
                });
            }
        }
    }

    let path = String::from("./tmp/img.png");
    let _ = image.save(&path).unwrap();
    return path;
}
