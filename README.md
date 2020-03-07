# check-email-exists

[![Build Status](https://travis-ci.com/ZiNai/check-email-exists.svg?branch=master)](https://travis-ci.com/ZiNai/check-email-exists)

Check if an email address exists before sending the email.
Neon-binding Rust crates "[`check-if-email-exists`](https://github.com/amaurymartiny/check-if-email-exists)".


## Features

From the check-if-email-exists repo [`README.md`](https://github.com/amaurymartiny/check-if-email-exists/edit/master/README.md) get more details.

✅ **Email deliverability:** Is an email for this address deliverable?

However, it goes more into details, and checks all the following properties of an email address:

✔️ **Syntax validation.** Is the address syntactically valid?

✔️ **DNS records validation.** Does the domain of the email address have valid MX DNS records?

✔️ **Disposable email address (DEA) validation.** Is the address provided by a known [disposable email address](https://en.wikipedia.org/wiki/Disposable_email_address) provider?

✔️ **SMTP server validation.** Can the mail exchanger of the email address domain be contacted successfully?

✔️ **Mailbox disabled.** Has this email address been disabled by the email provider?

✔️ **Full inbox.** Is the inbox of this mailbox full?

✔️ **Catch-all address.** Is this email address a [catch-all](https://debounce.io/blog/help/what-is-a-catch-all-or-accept-all/) address?

## Output Example

```json
{
	"input": "someone@gmail.com",
	"misc": {
		"is_disposable": false
	},
	"mx": {
		"records": [
			"alt3.gmail-smtp-in.l.google.com.",
			"gmail-smtp-in.l.google.com.",
			"alt1.gmail-smtp-in.l.google.com.",
			"alt4.gmail-smtp-in.l.google.com.",
			"alt2.gmail-smtp-in.l.google.com."
		]
	},
	"smtp": {
		"has_full_inbox": false,
		"is_catch_all": false,
		"is_deliverable": false,
		"is_disabled": true
	},
	"syntax": {
		"address": "someone@gmail.com",
		"domain": "gmail.com",
		"username": "someone",
		"valid_format": true
	}
}
```

## Installation

```bash
npm install check-email-exists
```

## Usage

```js
import { checkEmailExists ,checkEmailExistsSync} from 'check-email-exists';

// async check the email
await checkEmailExists("toEmail@gmail.com");

// sync check the email
checkEmailExistsSync("toEmail@gmail.com","fromEmail@gmail.com");
```

## Local Setup

```bash
git clone https://github.com/ZiNai/check-email-exists
cd check-email-exists
npm i
npm test
```
