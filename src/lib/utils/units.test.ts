import { describe, it, expect } from "vitest";
import {
  getUnitCategory,
  normalizeUnit,
  convertQuantity,
  aggregateQuantities,
  formatQuantity,
} from "./units";

describe("units", () => {
  describe("getUnitCategory", () => {
    it("identifies volume units", () => {
      expect(getUnitCategory("cup")).toBe("volume");
      expect(getUnitCategory("CUPS")).toBe("volume");
      expect(getUnitCategory("tbsp")).toBe("volume");
      expect(getUnitCategory("tsp")).toBe("volume");
      expect(getUnitCategory("ml")).toBe("volume");
      expect(getUnitCategory("L")).toBe("volume");
      expect(getUnitCategory("fl oz")).toBe("volume");
    });

    it("identifies weight units", () => {
      expect(getUnitCategory("g")).toBe("weight");
      expect(getUnitCategory("kg")).toBe("weight");
      expect(getUnitCategory("oz")).toBe("weight");
      expect(getUnitCategory("lb")).toBe("weight");
      expect(getUnitCategory("pounds")).toBe("weight");
    });

    it("identifies count units", () => {
      expect(getUnitCategory("")).toBe("count");
      expect(getUnitCategory("clove")).toBe("count");
      expect(getUnitCategory("cloves")).toBe("count");
      expect(getUnitCategory("can")).toBe("count");
      expect(getUnitCategory("bunch")).toBe("count");
    });

    it("returns other for unknown units", () => {
      expect(getUnitCategory("pinch")).toBe("other");
      expect(getUnitCategory("dash")).toBe("other");
      expect(getUnitCategory("handful")).toBe("other");
    });
  });

  describe("normalizeUnit", () => {
    it("normalizes volume units", () => {
      expect(normalizeUnit("cups")).toBe("cup");
      expect(normalizeUnit("C")).toBe("cup");
      expect(normalizeUnit("tablespoons")).toBe("tbsp");
      expect(normalizeUnit("teaspoons")).toBe("tsp");
      expect(normalizeUnit("liters")).toBe("L");
    });

    it("normalizes weight units", () => {
      expect(normalizeUnit("grams")).toBe("g");
      expect(normalizeUnit("kilograms")).toBe("kg");
      expect(normalizeUnit("ounces")).toBe("oz");
      expect(normalizeUnit("pounds")).toBe("lb");
      expect(normalizeUnit("lbs")).toBe("lb");
    });

    it("returns lowercase for unknown units", () => {
      expect(normalizeUnit("PINCH")).toBe("pinch");
    });
  });

  describe("convertQuantity", () => {
    it("converts between volume units", () => {
      // 2 cups to tbsp (1 cup = 16 tbsp)
      const result = convertQuantity(2, "cup", "tbsp");
      expect(result).not.toBeNull();
      expect(result!).toBeCloseTo(32, 0);
    });

    it("converts between weight units", () => {
      // 1 lb to oz (1 lb = 16 oz)
      const result = convertQuantity(1, "lb", "oz");
      expect(result).not.toBeNull();
      expect(result!).toBeCloseTo(16, 0);
    });

    it("returns null for incompatible units", () => {
      expect(convertQuantity(1, "cup", "lb")).toBeNull();
      expect(convertQuantity(1, "g", "ml")).toBeNull();
    });

    it("handles same unit conversion", () => {
      expect(convertQuantity(2.5, "cup", "cup")).toBeCloseTo(2.5, 5);
    });

    it("handles count units with same normalized form", () => {
      expect(convertQuantity(3, "clove", "cloves")).toBe(3);
    });

    it("returns null for different count units", () => {
      expect(convertQuantity(1, "clove", "head")).toBeNull();
    });
  });

  describe("aggregateQuantities", () => {
    it("aggregates same unit quantities", () => {
      const items = [
        { quantity: 1, unit: "cup" },
        { quantity: 0.5, unit: "cup" },
      ];
      const result = aggregateQuantities(items);
      expect(result).toHaveLength(1);
      expect(result[0].quantity).toBeCloseTo(1.5, 5);
      expect(result[0].unit).toBe("cup");
      expect(result[0].isConverted).toBe(false);
    });

    it("aggregates and converts different volume units", () => {
      const items = [
        { quantity: 1, unit: "cup" },
        { quantity: 2, unit: "tbsp" },
      ];
      const result = aggregateQuantities(items);
      expect(result).toHaveLength(1);
      // 1 cup + 2 tbsp ≈ 1.125 cups (2 tbsp = 0.125 cup)
      expect(result[0].quantity).toBeCloseTo(1.125, 1);
      expect(result[0].isConverted).toBe(true);
    });

    it("keeps incompatible units separate", () => {
      const items = [
        { quantity: 1, unit: "cup" },
        { quantity: 2, unit: "lb" },
      ];
      const result = aggregateQuantities(items);
      expect(result).toHaveLength(2);
    });

    it("aggregates count units by normalized form", () => {
      const items = [
        { quantity: 2, unit: "clove" },
        { quantity: 3, unit: "cloves" },
      ];
      const result = aggregateQuantities(items);
      expect(result).toHaveLength(1);
      expect(result[0].quantity).toBe(5);
    });

    it("keeps different count units separate", () => {
      const items = [
        { quantity: 2, unit: "clove" },
        { quantity: 1, unit: "head" },
      ];
      const result = aggregateQuantities(items);
      expect(result).toHaveLength(2);
    });

    it("handles empty input", () => {
      expect(aggregateQuantities([])).toEqual([]);
    });

    it("chooses most common unit as target", () => {
      // 2 items use tbsp, 1 uses cup - should convert to tbsp
      const items = [
        { quantity: 1, unit: "tbsp" },
        { quantity: 2, unit: "tbsp" },
        { quantity: 0.25, unit: "cup" }, // 0.25 cup = 4 tbsp
      ];
      const result = aggregateQuantities(items);
      expect(result).toHaveLength(1);
      expect(result[0].unit).toBe("tbsp");
      expect(result[0].quantity).toBeCloseTo(7, 0); // 1 + 2 + 4 = 7 tbsp
    });
  });

  describe("formatQuantity", () => {
    it("formats whole numbers without decimals", () => {
      expect(formatQuantity(2)).toBe("2");
      expect(formatQuantity(10)).toBe("10");
    });

    it("removes trailing zeros", () => {
      expect(formatQuantity(1.5)).toBe("1.5");
      expect(formatQuantity(2.0)).toBe("2");
    });

    it("rounds to avoid floating point artifacts", () => {
      expect(formatQuantity(1.9999999)).toBe("2");
      expect(formatQuantity(0.3333333)).toBe("0.33");
    });

    it("shows up to 2 decimal places when needed", () => {
      expect(formatQuantity(1.25)).toBe("1.25");
      expect(formatQuantity(0.125)).toBe("0.13"); // Rounded
    });
  });
});
