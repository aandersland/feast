//! Ingredient string parsing

use super::ParsedIngredient;
use html_escape::decode_html_entities;

/// Parse an ingredient string like "2 cups flour" into components
pub fn parse_ingredient(input: &str) -> ParsedIngredient {
    let decoded = decode_html_entities(input);
    let input = decoded.trim();

    // Try to parse quantity at the start
    let (quantity, rest) = parse_quantity(input);

    // Try to parse unit from the rest
    let (unit, name) = parse_unit(rest.trim());

    ParsedIngredient {
        quantity,
        unit: unit.to_string(),
        name: name.trim().to_string(),
    }
}

/// Parse quantity from the start of a string, handling fractions
fn parse_quantity(input: &str) -> (f64, &str) {
    let mut chars = input.char_indices().peekable();
    let mut end_idx = 0;
    let mut found_number = false;

    // Skip leading whitespace
    while let Some(&(_, c)) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }

    // Collect number characters (digits, fractions, decimals, spaces between)
    let start_idx = chars.peek().map(|(i, _)| *i).unwrap_or(0);

    while let Some(&(idx, c)) = chars.peek() {
        if c.is_ascii_digit() || c == '/' || c == '.' || c == '-' {
            found_number = true;
            end_idx = idx + c.len_utf8();
            chars.next();
        } else if c.is_whitespace() && found_number {
            // Check if next non-space is a digit or fraction (for "1 1/2")
            let mut lookahead = chars.clone();
            lookahead.next(); // skip space
            if let Some(&(_, next_c)) = lookahead.peek() {
                if next_c.is_ascii_digit() {
                    end_idx = idx + c.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if !found_number {
        return (0.0, input);
    }

    let num_str = &input[start_idx..end_idx];
    let quantity = parse_number_string(num_str);
    let rest = &input[end_idx..];

    (quantity, rest)
}

/// Parse a number string that may contain fractions
fn parse_number_string(s: &str) -> f64 {
    let s = s.trim();

    // Handle range (3-4) - take first number
    if let Some(dash_idx) = s.find('-') {
        if dash_idx > 0 {
            return parse_number_string(&s[..dash_idx]);
        }
    }

    // Handle mixed fraction (1 1/2)
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() == 2 {
        let whole: f64 = parts[0].parse().unwrap_or(0.0);
        let frac = parse_fraction(parts[1]);
        return whole + frac;
    }

    // Handle simple fraction (1/2)
    if s.contains('/') {
        return parse_fraction(s);
    }

    // Handle decimal or integer
    s.parse().unwrap_or(0.0)
}

/// Parse a fraction string like "1/2" to decimal
fn parse_fraction(s: &str) -> f64 {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].trim().parse().unwrap_or(0.0);
        let den: f64 = parts[1].trim().parse().unwrap_or(1.0);
        if den != 0.0 {
            return num / den;
        }
    }
    0.0
}

/// Parse unit from the start of a string
fn parse_unit(input: &str) -> (&str, &str) {
    let input = input.trim();
    let lower = input.to_lowercase();

    // Common units to recognize
    let units = [
        // Volume
        "cups",
        "cup",
        "c",
        "tablespoons",
        "tablespoon",
        "tbsp",
        "tbs",
        "tb",
        "teaspoons",
        "teaspoon",
        "tsp",
        "ts",
        "fluid ounces",
        "fluid ounce",
        "fl oz",
        "milliliters",
        "milliliter",
        "ml",
        "liters",
        "liter",
        "l",
        "pints",
        "pint",
        "pt",
        "quarts",
        "quart",
        "qt",
        "gallons",
        "gallon",
        "gal",
        // Weight
        "pounds",
        "pound",
        "lbs",
        "lb",
        "ounces",
        "ounce",
        "oz",
        "kilograms",
        "kilogram",
        "kg",
        "grams",
        "gram",
        "g",
        // Count
        "cloves",
        "clove",
        "slices",
        "slice",
        "pieces",
        "piece",
        "cans",
        "can",
        "bunches",
        "bunch",
        "heads",
        "head",
        "stalks",
        "stalk",
        "sprigs",
        "sprig",
        "packages",
        "package",
        "pkg",
        "pinches",
        "pinch",
        "dashes",
        "dash",
        // Size modifiers that act as units
        "large",
        "medium",
        "small",
    ];

    for unit in &units {
        if lower.starts_with(unit) {
            let unit_len = unit.len();
            // Make sure it's a word boundary
            let next_char = input.chars().nth(unit_len);
            if next_char.is_none()
                || next_char.unwrap().is_whitespace()
                || next_char.unwrap() == ','
            {
                return (&input[..unit_len], &input[unit_len..]);
            }
        }
    }

    // No recognized unit - check if first word looks like a unit (parenthetical)
    if input.starts_with('(') {
        // Handle "(15 oz) can" - skip parenthetical
        if let Some(close_paren) = input.find(')') {
            let after = &input[close_paren + 1..].trim_start();
            return parse_unit(after);
        }
    }

    ("", input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_ingredient() {
        let result = parse_ingredient("2 cups flour");
        assert_eq!(result.quantity, 2.0);
        assert_eq!(result.unit, "cups");
        assert_eq!(result.name, "flour");
    }

    #[test]
    fn test_fraction_ingredient() {
        let result = parse_ingredient("1/2 tsp salt");
        assert_eq!(result.quantity, 0.5);
        assert_eq!(result.unit, "tsp");
        assert_eq!(result.name, "salt");
    }

    #[test]
    fn test_mixed_fraction() {
        let result = parse_ingredient("1 1/2 cups sugar");
        assert_eq!(result.quantity, 1.5);
        assert_eq!(result.unit, "cups");
        assert_eq!(result.name, "sugar");
    }

    #[test]
    fn test_range_quantity() {
        let result = parse_ingredient("3-4 cloves garlic");
        assert_eq!(result.quantity, 3.0);
        assert_eq!(result.unit, "cloves");
        assert_eq!(result.name, "garlic");
    }

    #[test]
    fn test_no_quantity() {
        let result = parse_ingredient("salt to taste");
        assert_eq!(result.quantity, 0.0);
        assert_eq!(result.unit, "");
        assert_eq!(result.name, "salt to taste");
    }

    #[test]
    fn test_no_unit() {
        let result = parse_ingredient("2 eggs");
        assert_eq!(result.quantity, 2.0);
        assert_eq!(result.unit, "");
        assert_eq!(result.name, "eggs");
    }

    #[test]
    fn test_parenthetical_unit() {
        let result = parse_ingredient("1 (15 oz) can beans");
        assert_eq!(result.quantity, 1.0);
        assert_eq!(result.unit, "can");
        assert_eq!(result.name, "beans");
    }

    #[test]
    fn test_decimal_quantity() {
        let result = parse_ingredient("0.5 lb ground beef");
        assert_eq!(result.quantity, 0.5);
        assert_eq!(result.unit, "lb");
        assert_eq!(result.name, "ground beef");
    }
}
