const addon = require("..");

describe("basic", () => {
  it("works with async/await", async () => {
    expect.hasAssertions();
    const data = await addon.checkEmailExists("someone@gmail.com");
    expect(typeof data).toEqual("object");
    expect(data.smtp.is_deliverable).toEqual(false);
  });

  it("should return a JSON obj", () => {
    const data = addon.checkEmailExistsSync("someone@gmail.com");
    expect(typeof data).toEqual("object");
  });

  it("example email verified fail", () => {
    const data = addon.checkEmailExistsSync("someone@gmail.com");
    expect(data.smtp.is_deliverable).toEqual(false);
  });

  it("should fail if email not given", () => {
    expect(() => {
      addon.checkEmailExistsSync();
    }).toThrow();
  });
});
