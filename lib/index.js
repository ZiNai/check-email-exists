var addon = require("../native");
const { promisify } = require("util");

const checkEmailExistsPy = promisify(addon.checkEmailExistsAsync);

async function checkEmailExists(...args) {
  try {
    const toEmail = args.shift();
    const fromEmail = args.length > 0 ? args.shift() : null;
    let ret;
    if (!fromEmail) {
      ret = await checkEmailExistsPy(toEmail);
    } else {
      ret = await checkEmailExistsPy(toEmail, fromEmail);
    }

    return JSON.parse(ret);
  } catch (error) {
    console.log(error);
  }
}
module.exports = {
  checkEmailExists,
  checkEmailExistsSync: addon.checkEmailExistsSync
};
