/**
 * Unit conversion utilities for ingredient aggregation
 * Mirrors the Rust implementation in src-tauri/src/utils/units.rs
 */

export type UnitCategory = "volume" | "weight" | "count" | "other";

/**
 * Get the category for a unit
 */
export function getUnitCategory(unit: string): UnitCategory {
  const unitLower = unit.toLowerCase();

  // Volume units
  if (
    [
      "cup",
      "cups",
      "c",
      "tablespoon",
      "tablespoons",
      "tbsp",
      "tbs",
      "teaspoon",
      "teaspoons",
      "tsp",
      "ml",
      "milliliter",
      "milliliters",
      "l",
      "liter",
      "liters",
      "fl oz",
      "fluid ounce",
      "fluid ounces",
      "pint",
      "pints",
      "pt",
      "quart",
      "quarts",
      "qt",
      "gallon",
      "gallons",
      "gal",
    ].includes(unitLower)
  ) {
    return "volume";
  }

  // Weight units
  if (
    [
      "g",
      "gram",
      "grams",
      "kg",
      "kilogram",
      "kilograms",
      "oz",
      "ounce",
      "ounces",
      "lb",
      "lbs",
      "pound",
      "pounds",
    ].includes(unitLower)
  ) {
    return "weight";
  }

  // Count units
  if (
    [
      "",
      "whole",
      "piece",
      "pieces",
      "clove",
      "cloves",
      "slice",
      "slices",
      "can",
      "cans",
      "bunch",
      "bunches",
      "head",
      "heads",
      "stalk",
      "stalks",
      "sprig",
      "sprigs",
    ].includes(unitLower)
  ) {
    return "count";
  }

  return "other";
}

/**
 * Conversion factors to a base unit within each category
 * Volume: base = ml
 * Weight: base = g
 */
function getConversionFactor(unit: string): number | null {
  const unitLower = unit.toLowerCase();

  // Volume to ml
  const volumeFactors: Record<string, number> = {
    ml: 1.0,
    milliliter: 1.0,
    milliliters: 1.0,
    l: 1000.0,
    liter: 1000.0,
    liters: 1000.0,
    tsp: 4.929,
    teaspoon: 4.929,
    teaspoons: 4.929,
    tbsp: 14.787,
    tbs: 14.787,
    tablespoon: 14.787,
    tablespoons: 14.787,
    "fl oz": 29.574,
    "fluid ounce": 29.574,
    "fluid ounces": 29.574,
    cup: 236.588,
    cups: 236.588,
    c: 236.588,
    pint: 473.176,
    pints: 473.176,
    pt: 473.176,
    quart: 946.353,
    quarts: 946.353,
    qt: 946.353,
    gallon: 3785.41,
    gallons: 3785.41,
    gal: 3785.41,
  };

  // Weight to g
  const weightFactors: Record<string, number> = {
    g: 1.0,
    gram: 1.0,
    grams: 1.0,
    kg: 1000.0,
    kilogram: 1000.0,
    kilograms: 1000.0,
    oz: 28.3495,
    ounce: 28.3495,
    ounces: 28.3495,
    lb: 453.592,
    lbs: 453.592,
    pound: 453.592,
    pounds: 453.592,
  };

  return volumeFactors[unitLower] ?? weightFactors[unitLower] ?? null;
}

/**
 * Normalize a unit to its base form for display
 */
export function normalizeUnit(unit: string): string {
  const unitLower = unit.toLowerCase();
  const normalizations: Record<string, string> = {
    // Volume
    c: "cup",
    cups: "cup",
    tbs: "tbsp",
    tablespoons: "tbsp",
    tablespoon: "tbsp",
    teaspoons: "tsp",
    teaspoon: "tsp",
    milliliters: "ml",
    milliliter: "ml",
    liters: "L",
    liter: "L",
    "fluid ounces": "fl oz",
    "fluid ounce": "fl oz",
    pints: "pint",
    pt: "pint",
    quarts: "quart",
    qt: "quart",
    gallons: "gallon",
    gal: "gallon",
    // Weight
    grams: "g",
    gram: "g",
    kilograms: "kg",
    kilogram: "kg",
    ounces: "oz",
    ounce: "oz",
    pounds: "lb",
    pound: "lb",
    lbs: "lb",
    // Count - normalize plurals to singular
    pieces: "",
    piece: "",
    cloves: "clove",
    slices: "slice",
    cans: "can",
    bunches: "bunch",
    heads: "head",
    stalks: "stalk",
    sprigs: "sprig",
  };

  return normalizations[unitLower] ?? unitLower;
}

/**
 * Convert a quantity from one unit to another
 * Returns null if units are incompatible
 */
export function convertQuantity(
  quantity: number,
  fromUnit: string,
  toUnit: string
): number | null {
  const fromCat = getUnitCategory(fromUnit);
  const toCat = getUnitCategory(toUnit);

  // Must be same category
  if (fromCat !== toCat) {
    return null;
  }

  // Count units don't convert
  if (fromCat === "count" || fromCat === "other") {
    if (normalizeUnit(fromUnit) === normalizeUnit(toUnit)) {
      return quantity;
    }
    return null;
  }

  const fromFactor = getConversionFactor(fromUnit);
  const toFactor = getConversionFactor(toUnit);

  if (fromFactor === null || toFactor === null) {
    return null;
  }

  // Convert: from_unit -> base -> to_unit
  return (quantity * fromFactor) / toFactor;
}

export interface AggregatedQuantity {
  quantity: number;
  unit: string;
  isConverted: boolean;
}

/**
 * Find the most common unit in a group (for choosing target unit)
 */
function findBestTargetUnit(items: Array<{ quantity: number; unit: string }>): string {
  const counts = new Map<string, number>();

  for (const { unit } of items) {
    const normalized = normalizeUnit(unit);
    counts.set(normalized, (counts.get(normalized) ?? 0) + 1);
  }

  let bestUnit = "";
  let maxCount = 0;

  for (const [unit, count] of counts) {
    if (count > maxCount) {
      maxCount = count;
      bestUnit = unit;
    }
  }

  return bestUnit;
}

/**
 * Aggregate multiple quantities of the same ingredient
 * Returns the best unit and total quantity, or separate entries if incompatible
 */
export function aggregateQuantities(
  items: Array<{ quantity: number; unit: string }>
): AggregatedQuantity[] {
  if (items.length === 0) {
    return [];
  }

  // Group by unit category
  const byCategory = new Map<UnitCategory, Array<{ quantity: number; unit: string }>>();

  for (const item of items) {
    const cat = getUnitCategory(item.unit);
    const group = byCategory.get(cat) ?? [];
    group.push(item);
    byCategory.set(cat, group);
  }

  const results: AggregatedQuantity[] = [];

  for (const [category, group] of byCategory) {
    if (category === "count" || category === "other") {
      // For count/other, group by normalized unit
      const byUnit = new Map<string, number>();
      for (const { quantity, unit } of group) {
        const normalized = normalizeUnit(unit);
        byUnit.set(normalized, (byUnit.get(normalized) ?? 0) + quantity);
      }
      for (const [unit, qty] of byUnit) {
        results.push({
          quantity: qty,
          unit,
          isConverted: false,
        });
      }
    } else {
      // For volume/weight, convert to most common unit
      const targetUnit = findBestTargetUnit(group);
      let total = 0;
      let anyConverted = false;

      for (const { quantity, unit } of group) {
        const converted = convertQuantity(quantity, unit, targetUnit);
        if (converted !== null) {
          total += converted;
          if (normalizeUnit(unit) !== normalizeUnit(targetUnit)) {
            anyConverted = true;
          }
        }
      }

      results.push({
        quantity: total,
        unit: normalizeUnit(targetUnit),
        isConverted: anyConverted,
      });
    }
  }

  return results;
}

/**
 * Round a quantity for display, avoiding floating point artifacts
 * Shows up to 2 decimal places, but removes trailing zeros
 */
export function formatQuantity(quantity: number): string {
  // Round to 2 decimal places to avoid floating point display issues
  const rounded = Math.round(quantity * 100) / 100;

  // Format without trailing zeros
  if (rounded === Math.floor(rounded)) {
    return rounded.toString();
  }

  // Check if one decimal place is enough
  const oneDecimal = Math.round(quantity * 10) / 10;
  if (Math.abs(rounded - oneDecimal) < 0.001) {
    return oneDecimal.toString();
  }

  return rounded.toString();
}
