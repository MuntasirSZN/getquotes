use crate::config::{BoxCorners, Config, Layout};
use colored::Colorize;
use log::warn;
use std::ops::Range;

const FALLBACK_COLOR: RgbColor = RgbColor {
    r: 0x1E,
    g: 0x90,
    b: 0xFF,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Fill {
    Solid(RgbColor),
    Gradient(Vec<RgbColor>),
    Rainbow,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Decorations {
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    dimmed: bool,
    reversed: bool,
}

impl Decorations {
    fn apply_override(&mut self, other: Decorations) {
        self.bold |= other.bold;
        self.italic |= other.italic;
        self.underline |= other.underline;
        self.strikethrough |= other.strikethrough;
        self.dimmed |= other.dimmed;
        self.reversed |= other.reversed;
    }

    fn is_plain(&self) -> bool {
        !self.bold
            && !self.italic
            && !self.underline
            && !self.strikethrough
            && !self.dimmed
            && !self.reversed
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct TextStyle {
    decorations: Decorations,
    fill: Option<Fill>,
}

#[derive(Clone, Debug)]
struct StyleOverride {
    range: Range<usize>,
    style: TextStyle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FillSource {
    None,
    Base,
    Override(usize),
}

#[derive(Clone, Copy)]
struct BoxChars {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
}

pub fn render_output(cfg: &Config, quote: &str, author: &str) -> String {
    match cfg.layout {
        Layout::Default => render_default_layout(cfg, quote, author),
        Layout::Box => render_box_layout(cfg, quote, author),
    }
}

fn render_default_layout(cfg: &Config, quote: &str, author: &str) -> String {
    let quote_plain = format!("\"{quote}\"");
    let quote_style = resolve_quote_style(cfg);
    let nested_overrides = nested_quote_overrides(quote, &cfg.nested_quote_style);
    let styled_quote = style_text(&quote_plain, &quote_style, &nested_overrides);

    let author_style = resolve_author_style(cfg);
    let padded_dash = format!("{:>99}", "- ");
    let styled_dash = style_text(&padded_dash, &author_style, &[]);
    let styled_author = style_text(author, &author_style, &[]);

    format!("{styled_quote}\n\n {styled_dash}{styled_author}")
}

fn render_box_layout(cfg: &Config, quote: &str, author: &str) -> String {
    let quote_plain = format!("\"{quote}\"");
    let author_plain = author.to_string();
    let width = quote_plain
        .chars()
        .count()
        .max(author_plain.chars().count());

    let quote_style = resolve_quote_style(cfg);
    let nested_overrides = nested_quote_overrides(quote, &cfg.nested_quote_style);
    let quote_row_plain = pad_right(&quote_plain, width);
    let styled_quote = style_text(&quote_row_plain, &quote_style, &nested_overrides);

    let author_style = resolve_author_style(cfg);
    let author_row_plain = pad_left(&author_plain, width);
    let styled_author = style_text(&author_row_plain, &author_style, &[]);

    let chars = box_chars(cfg.box_corners);
    let top = format!(
        "{}{}{}",
        chars.top_left,
        chars.horizontal.to_string().repeat(width),
        chars.top_right
    );
    let bottom = format!(
        "{}{}{}",
        chars.bottom_left,
        chars.horizontal.to_string().repeat(width),
        chars.bottom_right
    );

    format!(
        "{top}\n{}{}{}\n{}{}{}\n{bottom}",
        chars.vertical, styled_quote, chars.vertical, chars.vertical, styled_author, chars.vertical
    )
}

fn resolve_quote_style(cfg: &Config) -> TextStyle {
    let mut style = parse_style_spec(&cfg.quote_style);
    if cfg.rainbow_mode {
        style.fill = Some(Fill::Rainbow);
        return style;
    }

    if style.fill.is_none() {
        style.fill = parse_fill_spec(&cfg.theme_color).or_else(|| {
            warn!(
                "Invalid color setting '{}'. Using fallback color.",
                cfg.theme_color
            );
            Some(Fill::Solid(FALLBACK_COLOR))
        });
    }

    style
}

fn resolve_author_style(cfg: &Config) -> TextStyle {
    let mut style = parse_style_spec(&cfg.author_style);
    if style.fill.is_none() {
        style.fill = Some(Fill::Solid(
            named_color_rgb("green").unwrap_or(FALLBACK_COLOR),
        ));
    }
    style
}

fn nested_quote_overrides(quote: &str, nested_quote_style: &str) -> Vec<StyleOverride> {
    if nested_quote_style.trim().is_empty() {
        return Vec::new();
    }

    let style = parse_style_spec(nested_quote_style);
    if style == TextStyle::default() {
        return Vec::new();
    }

    find_nested_quote_ranges(quote)
        .into_iter()
        .map(|range| StyleOverride {
            range: (range.start + 1)..(range.end + 1),
            style: style.clone(),
        })
        .collect()
}

fn style_text(text: &str, base_style: &TextStyle, overrides: &[StyleOverride]) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return String::new();
    }

    let mut decorations = vec![base_style.decorations; chars.len()];
    let mut fill_sources = vec![
        if base_style.fill.is_some() {
            FillSource::Base
        } else {
            FillSource::None
        };
        chars.len()
    ];

    for (override_idx, style_override) in overrides.iter().enumerate() {
        let end = style_override.range.end.min(chars.len());
        for idx in style_override.range.start.min(chars.len())..end {
            decorations[idx].apply_override(style_override.style.decorations);
            if style_override.style.fill.is_some() {
                fill_sources[idx] = FillSource::Override(override_idx);
            }
        }
    }

    let mut colors = vec![None; chars.len()];
    if let Some(fill) = &base_style.fill {
        let indices: Vec<_> = fill_sources
            .iter()
            .enumerate()
            .filter_map(|(idx, source)| (*source == FillSource::Base).then_some(idx))
            .collect();
        assign_fill_colors(fill, &indices, &mut colors);
    }

    for (override_idx, style_override) in overrides.iter().enumerate() {
        if let Some(fill) = &style_override.style.fill {
            let indices: Vec<_> = fill_sources
                .iter()
                .enumerate()
                .filter_map(|(idx, source)| {
                    (*source == FillSource::Override(override_idx)).then_some(idx)
                })
                .collect();
            assign_fill_colors(fill, &indices, &mut colors);
        }
    }

    let mut output = String::new();
    for (idx, ch) in chars.into_iter().enumerate() {
        output.push_str(&style_char(ch, colors[idx], decorations[idx]));
    }
    output
}

fn style_char(ch: char, color: Option<RgbColor>, decorations: Decorations) -> String {
    if color.is_none() && decorations.is_plain() {
        return ch.to_string();
    }

    let mut styled = ch.to_string().normal();
    if let Some(rgb) = color {
        styled = styled.truecolor(rgb.r, rgb.g, rgb.b);
    }
    if decorations.bold {
        styled = styled.bold();
    }
    if decorations.italic {
        styled = styled.italic();
    }
    if decorations.underline {
        styled = styled.underline();
    }
    if decorations.strikethrough {
        styled = styled.strikethrough();
    }
    if decorations.dimmed {
        styled = styled.dimmed();
    }
    if decorations.reversed {
        styled = styled.reversed();
    }

    styled.to_string()
}

fn assign_fill_colors(fill: &Fill, indices: &[usize], colors: &mut [Option<RgbColor>]) {
    if indices.is_empty() {
        return;
    }

    match fill {
        Fill::Solid(color) => {
            for &idx in indices {
                colors[idx] = Some(*color);
            }
        }
        Fill::Gradient(stops) => {
            for (step, &idx) in indices.iter().enumerate() {
                colors[idx] = Some(gradient_color(stops, step, indices.len()));
            }
        }
        Fill::Rainbow => {
            for (step, &idx) in indices.iter().enumerate() {
                colors[idx] = Some(rainbow_color(step, indices.len()));
            }
        }
    }
}

fn gradient_color(stops: &[RgbColor], step: usize, total_steps: usize) -> RgbColor {
    if stops.is_empty() {
        return FALLBACK_COLOR;
    }
    if stops.len() == 1 || total_steps <= 1 {
        return stops[0];
    }

    let scaled = step as f32 * (stops.len() - 1) as f32 / (total_steps - 1) as f32;
    let lower_idx = scaled.floor() as usize;
    let upper_idx = lower_idx
        .min(stops.len() - 1)
        .saturating_add(1)
        .min(stops.len() - 1);
    let t = scaled - lower_idx as f32;
    interpolate_color(stops[lower_idx], stops[upper_idx], t)
}

fn rainbow_color(step: usize, total_steps: usize) -> RgbColor {
    let hue = if total_steps <= 1 {
        0.0
    } else {
        step as f32 * 360.0 / total_steps as f32
    };
    hsl_to_rgb(hue, 1.0, 0.5)
}

fn interpolate_color(start: RgbColor, end: RgbColor, t: f32) -> RgbColor {
    let lerp = |from: u8, to: u8| from as f32 + (to as f32 - from as f32) * t;
    RgbColor {
        r: lerp(start.r, end.r).round().clamp(0.0, 255.0) as u8,
        g: lerp(start.g, end.g).round().clamp(0.0, 255.0) as u8,
        b: lerp(start.b, end.b).round().clamp(0.0, 255.0) as u8,
    }
}

fn parse_style_spec(spec: &str) -> TextStyle {
    let mut style = TextStyle::default();
    for token in split_top_level(spec, ',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        match normalize_token(token).as_str() {
            "bold" => style.decorations.bold = true,
            "italic" => style.decorations.italic = true,
            "underline" => style.decorations.underline = true,
            "strikethrough" | "strike" => style.decorations.strikethrough = true,
            "dimmed" | "dim" => style.decorations.dimmed = true,
            "reversed" | "reverse" => style.decorations.reversed = true,
            _ => {
                if let Some(fill) = parse_fill_spec(token) {
                    style.fill = Some(fill);
                } else {
                    warn!("Ignoring unknown style token '{}'.", token);
                }
            }
        }
    }
    style
}

fn parse_fill_spec(spec: &str) -> Option<Fill> {
    let trimmed = spec.trim();
    if trimmed.eq_ignore_ascii_case("rainbow") {
        return Some(Fill::Rainbow);
    }

    if let Some((function_name, inner)) = function_name_and_args(trimmed)
        && is_supported_gradient_function(function_name)
    {
        let stops: Vec<_> = split_top_level(inner, ',')
            .into_iter()
            .filter_map(|stop| parse_gradient_stop(stop.trim()))
            .collect();

        return match stops.len() {
            0 => None,
            1 => Some(Fill::Solid(stops[0])),
            _ => Some(Fill::Gradient(stops)),
        };
    }

    parse_color_spec(trimmed).map(Fill::Solid)
}

fn parse_gradient_stop(stop: &str) -> Option<RgbColor> {
    parse_color_spec(stop).or_else(|| {
        extract_color_like_prefix(stop)
            .as_deref()
            .and_then(parse_color_spec)
    })
}

fn parse_color_spec(spec: &str) -> Option<RgbColor> {
    parse_hex_color(spec)
        .or_else(|| parse_rgb_function(spec, "rgb"))
        .or_else(|| parse_rgba_function(spec))
        .or_else(|| parse_hsl_function(spec))
        .or_else(|| named_color_rgb(spec))
}

fn parse_hex_color(spec: &str) -> Option<RgbColor> {
    let clean_hex = spec.trim().strip_prefix('#').unwrap_or(spec.trim());
    if clean_hex.len() != 6 {
        return None;
    }

    Some(RgbColor {
        r: u8::from_str_radix(&clean_hex[0..2], 16).ok()?,
        g: u8::from_str_radix(&clean_hex[2..4], 16).ok()?,
        b: u8::from_str_radix(&clean_hex[4..6], 16).ok()?,
    })
}

fn parse_rgb_function(spec: &str, function_name: &str) -> Option<RgbColor> {
    let inner = function_args(spec, function_name)?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 3 {
        return None;
    }

    Some(RgbColor {
        r: parse_rgb_channel(&parts[0])?,
        g: parse_rgb_channel(&parts[1])?,
        b: parse_rgb_channel(&parts[2])?,
    })
}

fn parse_rgba_function(spec: &str) -> Option<RgbColor> {
    let inner = function_args(spec, "rgba")?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 4 {
        return None;
    }

    let rgb = RgbColor {
        r: parse_rgb_channel(&parts[0])?,
        g: parse_rgb_channel(&parts[1])?,
        b: parse_rgb_channel(&parts[2])?,
    };
    let alpha = parse_alpha_channel(&parts[3])?;

    Some(RgbColor {
        r: (rgb.r as f32 * alpha).round().clamp(0.0, 255.0) as u8,
        g: (rgb.g as f32 * alpha).round().clamp(0.0, 255.0) as u8,
        b: (rgb.b as f32 * alpha).round().clamp(0.0, 255.0) as u8,
    })
}

fn parse_hsl_function(spec: &str) -> Option<RgbColor> {
    let inner = function_args(spec, "hsl")?;
    let parts = split_top_level(inner, ',');
    if parts.len() != 3 {
        return None;
    }

    let hue = parts[0].trim().parse::<f32>().ok()?;
    let saturation = parse_percentage(&parts[1])?;
    let lightness = parse_percentage(&parts[2])?;
    Some(hsl_to_rgb(hue, saturation, lightness))
}

fn parse_rgb_channel(value: &str) -> Option<u8> {
    let trimmed = value.trim();
    if let Some(number) = trimmed.strip_suffix('%') {
        let percentage = number.trim().parse::<f32>().ok()?;
        if !(0.0..=100.0).contains(&percentage) {
            return None;
        }
        return Some((percentage * 255.0 / 100.0).round() as u8);
    }

    let channel = trimmed.parse::<f32>().ok()?;
    if !(0.0..=255.0).contains(&channel) {
        return None;
    }
    Some(channel.round() as u8)
}

fn parse_alpha_channel(value: &str) -> Option<f32> {
    let trimmed = value.trim();
    if let Some(number) = trimmed.strip_suffix('%') {
        let percentage = number.trim().parse::<f32>().ok()?;
        if !(0.0..=100.0).contains(&percentage) {
            return None;
        }
        return Some((percentage / 100.0).clamp(0.0, 1.0));
    }

    let alpha = trimmed.parse::<f32>().ok()?;
    if (0.0..=1.0).contains(&alpha) {
        Some(alpha)
    } else if (0.0..=255.0).contains(&alpha) {
        Some((alpha / 255.0).clamp(0.0, 1.0))
    } else {
        None
    }
}

fn parse_percentage(value: &str) -> Option<f32> {
    let number = value.trim().strip_suffix('%')?.trim().parse::<f32>().ok()?;
    if !(0.0..=100.0).contains(&number) {
        return None;
    }
    Some(number / 100.0)
}

fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> RgbColor {
    let hue = hue.rem_euclid(360.0) / 360.0;
    let saturation = saturation.clamp(0.0, 1.0);
    let lightness = lightness.clamp(0.0, 1.0);

    if saturation == 0.0 {
        let value = (lightness * 255.0).round() as u8;
        return RgbColor {
            r: value,
            g: value,
            b: value,
        };
    }

    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - lightness * saturation
    };
    let p = 2.0 * lightness - q;

    let to_rgb = |mut t: f32| {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }

        let channel = if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        };

        (channel * 255.0).round() as u8
    };

    RgbColor {
        r: to_rgb(hue + 1.0 / 3.0),
        g: to_rgb(hue),
        b: to_rgb(hue - 1.0 / 3.0),
    }
}

fn named_color_rgb(name: &str) -> Option<RgbColor> {
    match normalize_token(name).as_str() {
        "black" => Some(RgbColor { r: 0, g: 0, b: 0 }),
        "red" => Some(RgbColor { r: 128, g: 0, b: 0 }),
        "green" => Some(RgbColor { r: 0, g: 128, b: 0 }),
        "yellow" => Some(RgbColor {
            r: 128,
            g: 128,
            b: 0,
        }),
        "blue" => Some(RgbColor { r: 0, g: 0, b: 128 }),
        "magenta" | "purple" => Some(RgbColor {
            r: 128,
            g: 0,
            b: 128,
        }),
        "cyan" => Some(RgbColor {
            r: 0,
            g: 128,
            b: 128,
        }),
        "white" => Some(RgbColor {
            r: 192,
            g: 192,
            b: 192,
        }),
        "brightblack" => Some(RgbColor {
            r: 128,
            g: 128,
            b: 128,
        }),
        "brightred" => Some(RgbColor { r: 255, g: 0, b: 0 }),
        "brightgreen" => Some(RgbColor { r: 0, g: 255, b: 0 }),
        "brightyellow" => Some(RgbColor {
            r: 255,
            g: 255,
            b: 0,
        }),
        "brightblue" => Some(RgbColor { r: 0, g: 0, b: 255 }),
        "brightmagenta" | "brightpurple" => Some(RgbColor {
            r: 255,
            g: 0,
            b: 255,
        }),
        "brightcyan" => Some(RgbColor {
            r: 0,
            g: 255,
            b: 255,
        }),
        "brightwhite" => Some(RgbColor {
            r: 255,
            g: 255,
            b: 255,
        }),
        _ => None,
    }
}

fn box_chars(corners: BoxCorners) -> BoxChars {
    match corners {
        BoxCorners::Pointy => BoxChars {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            horizontal: '-',
            vertical: '|',
        },
        BoxCorners::Rounded => BoxChars {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            horizontal: '─',
            vertical: '│',
        },
    }
}

fn pad_right(value: &str, width: usize) -> String {
    let padding = width.saturating_sub(value.chars().count());
    format!("{value}{}", " ".repeat(padding))
}

fn pad_left(value: &str, width: usize) -> String {
    let padding = width.saturating_sub(value.chars().count());
    format!("{}{value}", " ".repeat(padding))
}

fn find_nested_quote_ranges(text: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut ascii_start = None;
    let mut curly_start = None;

    for (idx, ch) in text.chars().enumerate() {
        match ch {
            '"' => {
                if let Some(start) = ascii_start.take() {
                    ranges.push(start..(idx + 1));
                } else {
                    ascii_start = Some(idx);
                }
            }
            '“' => curly_start = Some(idx),
            '”' => {
                if let Some(start) = curly_start.take() {
                    ranges.push(start..(idx + 1));
                }
            }
            _ => {}
        }
    }

    ranges
}

fn function_args<'a>(value: &'a str, function_name: &str) -> Option<&'a str> {
    let trimmed = value.trim();
    let prefix = format!("{function_name}(");
    if !trimmed
        .get(..prefix.len())?
        .eq_ignore_ascii_case(prefix.as_str())
        || !trimmed.ends_with(')')
    {
        return None;
    }

    trimmed.get(prefix.len()..trimmed.len().saturating_sub(1))
}

fn function_name_and_args(value: &str) -> Option<(&str, &str)> {
    let trimmed = value.trim();
    let open_idx = trimmed.find('(')?;
    let close_idx = trimmed.rfind(')')?;
    if close_idx <= open_idx {
        return None;
    }

    Some((
        trimmed.get(..open_idx)?.trim(),
        trimmed.get((open_idx + 1)..close_idx)?.trim(),
    ))
}

fn is_supported_gradient_function(name: &str) -> bool {
    matches!(
        normalize_token(name).as_str(),
        "lineargradient"
            | "radialgradient"
            | "conicgradient"
            | "repeatinglineargradient"
            | "repeatingradialgradient"
            | "repeatingconicgradient"
    )
}

fn extract_color_like_prefix(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(open_idx) = trimmed.find('(') {
        let function_name = trimmed.get(..open_idx)?.trim();
        if parse_color_spec(function_name).is_some() {
            return Some(function_name.to_string());
        }

        if matches!(
            normalize_token(function_name).as_str(),
            "rgb" | "rgba" | "hsl"
        ) {
            let mut depth = 0usize;
            for (idx, ch) in trimmed.char_indices() {
                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            return trimmed.get(..=idx).map(str::to_string);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    trimmed
        .split_whitespace()
        .next()
        .map(str::trim)
        .filter(|token| parse_color_spec(token).is_some())
        .map(str::to_string)
}

fn split_top_level(value: &str, delimiter: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;

    for ch in value.chars() {
        match ch {
            '(' => {
                depth += 1;
                current.push(ch);
            }
            ')' => {
                depth = depth.saturating_sub(1);
                current.push(ch);
            }
            _ if ch == delimiter && depth == 0 => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    parts
}

fn normalize_token(token: &str) -> String {
    token
        .trim()
        .to_ascii_lowercase()
        .replace(['-', '_', ' '], "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        BoxCorners, Config, Layout, default_api_calls_per_minute, default_author_style,
        default_authors, default_box_corners, default_log_file, default_max_tries,
        default_nested_quote_style, default_prefer_cache, default_quote_style, default_theme_color,
    };

    fn sample_config() -> Config {
        Config {
            authors: default_authors(),
            theme_color: default_theme_color(),
            quote_style: default_quote_style(),
            author_style: default_author_style(),
            nested_quote_style: default_nested_quote_style(),
            max_tries: default_max_tries(),
            log_file: default_log_file(),
            rainbow_mode: false,
            layout: Layout::Default,
            box_corners: default_box_corners(),
            prefer_cache: default_prefer_cache(),
            api_calls_per_minute: default_api_calls_per_minute(),
        }
    }

    fn strip_ansi(value: &str) -> String {
        let mut output = String::new();
        let mut chars = value.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\u{1b}' && chars.peek() == Some(&'[') {
                chars.next();
                for next in chars.by_ref() {
                    if next == 'm' {
                        break;
                    }
                }
            } else {
                output.push(ch);
            }
        }

        output
    }

    #[test]
    fn parses_supported_color_formats() {
        assert_eq!(
            parse_color_spec("#FF0000"),
            Some(RgbColor { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            parse_color_spec("rgb(10, 20, 30)"),
            Some(RgbColor {
                r: 10,
                g: 20,
                b: 30
            })
        );
        assert_eq!(
            parse_color_spec("rgba(255, 0, 0, 0.5)"),
            Some(RgbColor { r: 128, g: 0, b: 0 })
        );
        assert_eq!(
            parse_color_spec("hsl(120, 100%, 50%)"),
            Some(RgbColor { r: 0, g: 255, b: 0 })
        );
    }

    #[test]
    fn parses_gradient_style_token() {
        let fill = parse_fill_spec("linear-gradient(#ff0000, rgb(0, 255, 0), hsl(240, 100%, 50%))");
        assert!(matches!(fill, Some(Fill::Gradient(stops)) if stops.len() == 3));
    }

    #[test]
    fn parses_css_inspired_gradient_variants() {
        let radial_fill = parse_fill_spec(
            "radial-gradient(circle at center, #ff0000 0%, rgb(0, 255, 0) 50%, blue 100%)",
        );
        assert!(matches!(radial_fill, Some(Fill::Gradient(stops)) if stops.len() == 3));

        let conic_fill = parse_fill_spec(
            "conic-gradient(from 90deg at center, red 0deg, yellow 120deg, blue 240deg)",
        );
        assert!(matches!(conic_fill, Some(Fill::Gradient(stops)) if stops.len() == 3));

        let repeating_fill = parse_fill_spec(
            "repeating-radial-gradient(circle, hsl(0, 100%, 50%) 0 10%, hsl(240, 100%, 50%) 10% 20%)",
        );
        assert!(matches!(repeating_fill, Some(Fill::Gradient(stops)) if stops.len() == 2));
    }

    #[test]
    fn extracts_color_prefix_from_gradient_stop() {
        assert_eq!(
            parse_gradient_stop("rgb(255, 0, 0) 25%"),
            Some(RgbColor { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            parse_gradient_stop("blue 10% 90%"),
            Some(RgbColor { r: 0, g: 0, b: 128 })
        );
        assert_eq!(parse_gradient_stop("to right"), None);
    }

    #[test]
    fn default_layout_adds_space_after_dash() {
        let rendered = render_output(&sample_config(), "Hello", "Author");
        assert!(strip_ansi(&rendered).contains("- Author"));
    }

    #[test]
    fn box_layout_supports_rounded_corners() {
        let mut config = sample_config();
        config.layout = Layout::Box;
        config.box_corners = BoxCorners::Rounded;

        let rendered = render_output(&config, "Hi", "Author");
        assert!(rendered.contains('╭'));
        assert!(rendered.contains('╯'));
        assert!(rendered.contains('│'));
    }

    #[test]
    fn finds_nested_quote_ranges() {
        assert_eq!(find_nested_quote_ranges(r#"He said "hi""#), vec![8..12]);
    }
}
