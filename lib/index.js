var addon = require("../native");
var now = new Date();
console.log(addon.check_email_exists("313670398@gmail.com"));
console.log("coast:", new Date() - now);
