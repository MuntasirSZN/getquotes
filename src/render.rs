use crate::config::{BoxCorners, Config, Layout};
use anstyle::{Color as AnsiColor, RgbColor as AnsiRgbColor, Style as AnsiStyle};
use cssparser::{ParseError, Parser, ParserInput, Token};
use cssparser_color::{Color as CssColor, parse_color_keyword};
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

#[derive(Clone, Copy, Debug)]
enum NumericComponent {
    Number(f32),
    Percentage(f32),
    Angle(f32, AngleUnit),
}

#[derive(Clone, Copy, Debug)]
enum AngleUnit {
    Deg,
    Grad,
    Rad,
    Turn,
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

    let max_quote_width = terminal_inner_width();
    let quote_lines = wrap_text_lines(&quote_plain, max_quote_width);
    let width = quote_lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
        .max(author_plain.chars().count());

    let quote_style = resolve_quote_style(cfg);
    let quote_rows = quote_lines
        .iter()
        .map(|line| {
            let nested_overrides = nested_quote_overrides(line, &cfg.nested_quote_style);
            let quote_row_plain = pad_right(line, width);
            style_text(&quote_row_plain, &quote_style, &nested_overrides)
        })
        .collect::<Vec<_>>();

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

    let mut rows = quote_rows;
    rows.push(styled_author);
    let body = rows
        .iter()
        .map(|row| format!("{}{}{}", chars.vertical, row, chars.vertical))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{top}\n{body}\n{bottom}")
}

fn terminal_inner_width() -> usize {
    const BOX_BORDER_WIDTH: usize = 2;
    const DEFAULT_INNER_WIDTH: usize = 100;
    const MIN_INNER_WIDTH: usize = 20;

    std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .map(|width| width.saturating_sub(BOX_BORDER_WIDTH).max(MIN_INNER_WIDTH))
        .unwrap_or(DEFAULT_INNER_WIDTH)
}

fn wrap_text_lines(text: &str, max_width: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }

    if max_width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.is_empty() {
            push_wrapped_word(&mut lines, &mut current, word, max_width);
            continue;
        }

        let prospective_len = current.chars().count() + 1 + word.chars().count();
        if prospective_len <= max_width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            push_wrapped_word(&mut lines, &mut current, word, max_width);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn push_wrapped_word(lines: &mut Vec<String>, current: &mut String, word: &str, max_width: usize) {
    if word.chars().count() <= max_width {
        current.push_str(word);
        return;
    }

    if !current.is_empty() {
        lines.push(std::mem::take(current));
    }

    let mut chunk = String::new();
    for ch in word.chars() {
        chunk.push(ch);
        if chunk.chars().count() == max_width {
            lines.push(std::mem::take(&mut chunk));
        }
    }

    if !chunk.is_empty() {
        current.push_str(&chunk);
    }
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

    let mut style = AnsiStyle::new();
    if let Some(rgb) = color {
        style = style.fg_color(Some(AnsiColor::Rgb(AnsiRgbColor(rgb.r, rgb.g, rgb.b))));
    }
    if decorations.bold {
        style = style.bold();
    }
    if decorations.italic {
        style = style.italic();
    }
    if decorations.underline {
        style = style.underline();
    }
    if decorations.strikethrough {
        style = style.strikethrough();
    }
    if decorations.dimmed {
        style = style.dimmed();
    }
    if decorations.reversed {
        style = style.invert();
    }

    format!("{style}{ch}{style:#}")
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

    if let Some(stops) = parse_gradient_spec(trimmed) {
        return match stops.len() {
            0 => None,
            1 => Some(Fill::Solid(stops[0])),
            _ => Some(Fill::Gradient(stops)),
        };
    }

    parse_color_spec(trimmed).map(Fill::Solid)
}

fn parse_gradient_spec(spec: &str) -> Option<Vec<RgbColor>> {
    let mut input = ParserInput::new(spec);
    let mut parser = Parser::new(&mut input);
    let function_name = match parser.next().ok()? {
        Token::Function(name) if is_supported_gradient_function(name.as_ref()) => {
            name.as_ref().to_string()
        }
        _ => return None,
    };

    let stops = parse_gradient_function(&function_name, &mut parser)?;
    parser.expect_exhausted().ok()?;
    Some(stops)
}

fn parse_gradient_function<'i, 't>(
    function_name: &str,
    parser: &mut Parser<'i, 't>,
) -> Option<Vec<RgbColor>> {
    if !is_supported_gradient_function(function_name) {
        return None;
    }

    parser
        .parse_nested_block(|input| -> Result<Vec<RgbColor>, ParseError<'i, ()>> {
            Ok(input.parse_comma_separated_ignoring_errors(
                |stop| -> Result<RgbColor, ParseError<'i, ()>> {
                    parse_gradient_stop_value(stop).ok_or_else(|| stop.new_custom_error(()))
                },
            ))
        })
        .ok()
}

fn parse_color_spec(spec: &str) -> Option<RgbColor> {
    let mut input = ParserInput::new(spec);
    let mut parser = Parser::new(&mut input);
    let color = parse_single_color(&mut parser)?;
    parser.expect_exhausted().ok()?;
    Some(color)
}

#[cfg(test)]
fn parse_gradient_stop(spec: &str) -> Option<RgbColor> {
    let mut input = ParserInput::new(spec);
    let mut parser = Parser::new(&mut input);
    parse_gradient_stop_value(&mut parser)
}

fn parse_gradient_stop_value<'i, 't>(input: &mut Parser<'i, 't>) -> Option<RgbColor> {
    let mut color = None;

    while let Ok(token) = input.next().cloned() {
        if color.is_none() {
            color = parse_color_token(token, input);
        }
    }

    color
}

fn parse_single_color<'i, 't>(input: &mut Parser<'i, 't>) -> Option<RgbColor> {
    let token = input.next().ok()?.clone();
    parse_color_token(token, input)
}

fn parse_color_token<'i, 't>(token: Token<'i>, parser: &mut Parser<'i, 't>) -> Option<RgbColor> {
    match token {
        Token::Hash(value) | Token::IDHash(value) => parse_hex_color(value.as_ref()),
        Token::Ident(value) => named_color_rgb(value.as_ref()),
        Token::Function(name) => parser
            .parse_nested_block(|input| -> Result<Option<RgbColor>, ParseError<'i, ()>> {
                Ok(parse_color_function(name.as_ref(), input))
            })
            .ok()
            .flatten(),
        _ => None,
    }
}

fn parse_hex_color(value: &str) -> Option<RgbColor> {
    if value.len() != 6 {
        return None;
    }

    Some(RgbColor {
        r: u8::from_str_radix(&value[0..2], 16).ok()?,
        g: u8::from_str_radix(&value[2..4], 16).ok()?,
        b: u8::from_str_radix(&value[4..6], 16).ok()?,
    })
}

fn parse_color_function<'i, 't>(
    function_name: &str,
    input: &mut Parser<'i, 't>,
) -> Option<RgbColor> {
    match normalize_token(function_name).as_str() {
        "rgb" => parse_rgb_function(input, false),
        "rgba" => parse_rgb_function(input, true),
        "hsl" => parse_hsl_function(input),
        _ => None,
    }
}

fn parse_rgb_function<'i, 't>(input: &mut Parser<'i, 't>, allow_alpha: bool) -> Option<RgbColor> {
    let components = parse_numeric_components(input)?;
    let expected_len = if allow_alpha { 4 } else { 3 };
    if components.len() != expected_len {
        return None;
    }

    let rgb = RgbColor {
        r: parse_rgb_channel(components[0])?,
        g: parse_rgb_channel(components[1])?,
        b: parse_rgb_channel(components[2])?,
    };
    let alpha = if allow_alpha {
        parse_alpha_channel(components[3])?
    } else {
        1.0
    };

    Some(RgbColor {
        r: (rgb.r as f32 * alpha).round().clamp(0.0, 255.0) as u8,
        g: (rgb.g as f32 * alpha).round().clamp(0.0, 255.0) as u8,
        b: (rgb.b as f32 * alpha).round().clamp(0.0, 255.0) as u8,
    })
}

fn parse_hsl_function<'i, 't>(input: &mut Parser<'i, 't>) -> Option<RgbColor> {
    let components = parse_numeric_components(input)?;
    if components.len() != 3 {
        return None;
    }

    let hue = parse_hue_channel(components[0])?;
    let saturation = parse_percentage(components[1])?;
    let lightness = parse_percentage(components[2])?;
    Some(hsl_to_rgb(hue, saturation, lightness))
}

fn parse_numeric_components<'i, 't>(input: &mut Parser<'i, 't>) -> Option<Vec<NumericComponent>> {
    let mut components = Vec::new();

    while let Ok(token) = input.next().cloned() {
        match token {
            Token::Comma => {}
            Token::Number { value, .. } => components.push(NumericComponent::Number(value)),
            Token::Percentage { unit_value, .. } => {
                components.push(NumericComponent::Percentage(unit_value))
            }
            Token::Dimension { value, unit, .. } => components.push(NumericComponent::Angle(
                value,
                parse_angle_unit(unit.as_ref())?,
            )),
            _ => return None,
        }
    }

    Some(components)
}

fn parse_rgb_channel(value: NumericComponent) -> Option<u8> {
    match value {
        NumericComponent::Number(channel) if (0.0..=255.0).contains(&channel) => {
            Some(channel.round() as u8)
        }
        NumericComponent::Percentage(channel) if (0.0..=1.0).contains(&channel) => {
            Some((channel * 255.0).round() as u8)
        }
        _ => None,
    }
}

fn parse_alpha_channel(value: NumericComponent) -> Option<f32> {
    match value {
        NumericComponent::Number(alpha) if (0.0..=1.0).contains(&alpha) => Some(alpha),
        NumericComponent::Number(alpha) if (0.0..=255.0).contains(&alpha) => {
            Some((alpha / 255.0).clamp(0.0, 1.0))
        }
        NumericComponent::Percentage(alpha) if (0.0..=1.0).contains(&alpha) => Some(alpha),
        _ => None,
    }
}

fn parse_percentage(value: NumericComponent) -> Option<f32> {
    match value {
        NumericComponent::Percentage(percentage) if (0.0..=1.0).contains(&percentage) => {
            Some(percentage)
        }
        _ => None,
    }
}

fn parse_hue_channel(value: NumericComponent) -> Option<f32> {
    match value {
        NumericComponent::Number(hue) => Some(hue),
        NumericComponent::Angle(hue, AngleUnit::Deg) => Some(hue),
        NumericComponent::Angle(hue, AngleUnit::Grad) => Some(hue * 360.0 / 400.0),
        NumericComponent::Angle(hue, AngleUnit::Rad) => Some(hue.to_degrees()),
        NumericComponent::Angle(hue, AngleUnit::Turn) => Some(hue * 360.0),
        _ => None,
    }
}

fn parse_angle_unit(unit: &str) -> Option<AngleUnit> {
    match normalize_token(unit).as_str() {
        "deg" => Some(AngleUnit::Deg),
        "grad" => Some(AngleUnit::Grad),
        "rad" => Some(AngleUnit::Rad),
        "turn" => Some(AngleUnit::Turn),
        _ => None,
    }
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
    let normalized = normalize_token(name);
    let keyword = match normalized.as_str() {
        "brightblack" => "gray",
        "brightred" => "red",
        "brightgreen" => "lime",
        "brightyellow" => "yellow",
        "brightblue" => "blue",
        "brightmagenta" | "brightpurple" => "magenta",
        "brightcyan" => "cyan",
        "brightwhite" => "white",
        _ => normalized.as_str(),
    };

    match parse_color_keyword::<CssColor>(keyword).ok()? {
        CssColor::Rgba(rgba) => Some(RgbColor {
            r: (rgba.red as f32 * rgba.alpha).round().clamp(0.0, 255.0) as u8,
            g: (rgba.green as f32 * rgba.alpha).round().clamp(0.0, 255.0) as u8,
            b: (rgba.blue as f32 * rgba.alpha).round().clamp(0.0, 255.0) as u8,
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
            Some(RgbColor { r: 0, g: 0, b: 255 })
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
    fn box_layout_wraps_long_quotes() {
        const EXTRA_WIDTH: usize = 10;

        let mut config = sample_config();
        config.layout = Layout::Box;
        let quote = "a".repeat((terminal_inner_width() * 2) + EXTRA_WIDTH);

        let rendered = render_output(&config, &quote, "Author");
        let plain = strip_ansi(&rendered);
        let lines = plain.lines().collect::<Vec<_>>();

        assert!(lines.len() > 4);
        assert!(lines[0].chars().count() <= terminal_inner_width() + 2);
    }

    #[test]
    fn wraps_long_unbroken_words() {
        let lines = wrap_text_lines("abcdefghij", 4);
        assert_eq!(lines, vec!["abcd", "efgh", "ij"]);
    }

    #[test]
    fn finds_nested_quote_ranges() {
        assert_eq!(find_nested_quote_ranges(r#"He said "hi""#), vec![8..12]);
    }
}
