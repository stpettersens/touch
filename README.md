### touch
> :fu: Touch utility implementation.

[![Build Status](https://travis-ci.org/stpettersens/touch.png?branch=master)](https://travis-ci.org/stpettersens/touch)

```
Touch utility implementation.
Copyright 2017 Sam Saint-Pettersen.

Released under the MIT License.

Usage: touch <file> [options]

Options:

-c | --no-create: Do not create file if it does not exist.
-a | --access: Change the access time only.
-m | --modification: Change the modification time only.
-d | --date <iso8601>: Use ISO 8601 timestamp (e.g 2017-01-02T23:50:00+00:00).
-u | --unix <timestamp>: Use Unix timestamp (e.g. 1483402603).")
-r | --reference <ref_file>: Use reference file's time instead of current time.
-l | --log: Log used Unix timestamp to console for -d or -u option.

```
