% { bin | upper }(1) Version { version } | The git-mit suite of git hooks

NAME
====

**{ bin }** — { about }

SYNOPSIS
========

{ usage }

DESCRIPTION
===========

{ about }

Commands
--------

```
{subcommands|unescape}
```


Options
-------

```
{unified|unescape}
```

FILES
=====

*~/.config/git-mit/mit.yml*

:   Configuration file listing authors

```yaml
---
ae:
  name: Anyone Else
  email: anyone@example.com
bt:
  name: Billie Thompson
  email: billie@example.com
  signingkey: 0A46826A
se:
  name: Someone Else
  email: someone@example.com
```

*.git-mit.toml* or *.git-mit.toml.dist*

:   Committed lint configuration

You can add a `.git-mit.toml` or `.git-mit.toml.dist` to the root of
your repository and we will read it and try to enable the correct lints
(with `.git-mit.toml` taking precedence).

I recommend you commit `.git-mit.toml.dist` and `.gitignore`
`.git-mit.toml` to allow easy local reconfiguration

``` toml
[mit.lint]
"pivotal-tracker-id-missing" = true
```

BUGS
====

See GitHub Issues: <https://github.com/PurpleBooth/git-mit/issues>

AUTHOR
======

{ author }

SEE ALSO
========

**git-mit(1)**, **git-mit-config(1)**, **mit-commit-msg(1)**, **mit-pre-commit(1)**, **mit-prepare-commit-msg(1)**
