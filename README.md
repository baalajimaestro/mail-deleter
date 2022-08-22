# Mail Deleter

Filtering mails that come through different mail ids or from ids that also carry valuable info, can be hard

This simple program deletes mail based on all the info the `Show Original` section of your mail shows. You can choose the mailing list, or any parameter there and add it to `patterns.txt` before executing

You can add as many patterns as you wish, and all of these patterns are parsed as regex patterns, so feel free to do that as well.

**Required variables:**
- `IMAP_DOMAIN`: IMAP Server Domain
- `IMAP_PORT`: IMAP Server Port
- `IMAP_USERNAME`: IMAP Authentication Username
- `IMAP_PASSWORD`: IMAP Authentication Password
