const addon = require("..");

describe("basic", () => {
  // for build release success
  // this two case can`t pass in travis-ci`s linux os and coast too much time,
  // but quickly passed in oxs env.

  it("works with async/await", async () => {
    // expect.hasAssertions(2);
    const data = await addon.checkEmailExists("someone@gmail.com");
    expect(typeof data).toEqual("object");
    expect(data.smtp.is_deliverable).toEqual(false);
  });

  it("example email verified fail", () => {
    const data = addon.checkEmailExistsSync("someone@gmail.com");
    expect(data.smtp.is_deliverable).toEqual(false);
  });

  it("should return a JSON obj", () => {
    const data = addon.checkEmailExistsSync("someone@gmail.com");
    expect(typeof data).toEqual("object");
  });



  it("should fail if email not given", () => {
    expect(() => {
      addon.checkEmailExistsSync();
    }).toThrow();
  });
});
