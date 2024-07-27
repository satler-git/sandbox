import { describe, expect, test } from "vitest";
import { get } from "./hello";

describe("get", () => {
	test("should combine string with parameter value", async () => {
		const resp = await get({ name: "world" });
		expect(resp.message).toBe("Hello world!");
	});
});

describe("cat", () => {
	test("should combine string with parameter value", async () => {
		const resp = await get({ name: "world3" });
		expect(resp.message).toBe("Meow world! Meow world! Meow world!");
	});
});

describe("cat", () => {
	test("should combine string with parameter value", async ({ expect }) => {
		const error = new Error("The last letter is not aF number.");
		await expect(get({ name: "world" })).rejects.toThrow(error);
	});
});
