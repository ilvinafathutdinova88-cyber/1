import { describe, expect, test } from "bun:test";
import { speechDirection, translate } from "../src/ui/i18n.ts";

describe("interface language", () => {
  test("Arabic labels translate while English remains unchanged", () => {
    expect(translate("Speak", "ar")).toBe("تحدث");
    expect(translate("Speak", "en")).toBe("Speak");
  });

  test("direction follows the selected interface and unknown labels fall back", () => {
    expect(speechDirection("ar")).toBe("rtl");
    expect(speechDirection("en")).toBe("ltr");
    expect(translate("Unknown control", "ar")).toBe("Unknown control");
  });
});
