# Index of Lints

## Setup

As always we need a working it repository the, with the hooks installed
to run these lints

``` shell,script(name="1",expected_exit_code=0)
git init .
git mit-install
```

I'm going to assume you've run a `git mit` recently

``` shell,script(name="1",expected_exit_code=0)
git mit bt
```

### Lint list

You can see the full available lint list at any time by running

``` shell,script(name="lint-list",expected_exit_code=0)
git mit-config lint available
```

``` text,verify(script_name="lint-list",stream=stdout)
+-----------------------------------+----------+
| Lint                              | Status   |
+==============================================+
| duplicated-trailers               | enabled  |
|-----------------------------------+----------|
| pivotal-tracker-id-missing        | disabled |
|-----------------------------------+----------|
| jira-issue-key-missing            | disabled |
|-----------------------------------+----------|
| github-id-missing                 | disabled |
|-----------------------------------+----------|
| subject-not-separated-from-body   | enabled  |
|-----------------------------------+----------|
| subject-longer-than-72-characters | enabled  |
|-----------------------------------+----------|
| subject-line-not-capitalized      | disabled |
|-----------------------------------+----------|
| subject-line-ends-with-period     | disabled |
|-----------------------------------+----------|
| body-wider-than-72-characters     | enabled  |
|-----------------------------------+----------|
| not-conventional-commit           | disabled |
|-----------------------------------+----------|
| not-emoji-log                     | disabled |
+-----------------------------------+----------+
```

#### Trailers

Lints relating to trailers:

##### duplicated-trailers

Detect duplicated `Signed-off-by`, `Co-authored-by`, and `Relates-to`
Trailers.

###### Default status

On an empty repository

``` shell,script(name="duplicated-trailers-default",expected_exit_code=0)
git mit-config lint status duplicated-trailers
```

``` text,verify(script_name="duplicated-trailers-default",stream=stdout)
+---------------------+---------+
| Lint                | Status  |
+===============================+
| duplicated-trailers | enabled |
+---------------------+---------+
```

###### Valid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that has trailers and is valid

Co-authored-by: Billie Thompson <billie@example.com>
Signed-off-by: Someone Else <someone@example.com>
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that has trailers and is invalid

Co-authored-by: Billie Thompson <billie@example.com>
Co-authored-by: Billie Thompson <billie@example.com>
Signed-off-by: Someone Else <someone@example.com>
Signed-off-by: Someone Else <someone@example.com>
Relates-to: #315
Relates-to: #315
```

Committing will fail.

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
Demonstration Commit Message

This is a commit message that has trailers and is invalid

Co-authored-by: Billie Thompson <billie@example.com>
Co-authored-by: Billie Thompson <billie@example.com>
Signed-off-by: Someone Else <someone@example.com>
Signed-off-by: Someone Else <someone@example.com>
Relates-to: #315
Relates-to: #315


---

Your commit message has duplicated trailers

These are normally added accidentally when you're rebasing or amending to a commit, sometimes in the text editor, but often by git hooks.

You can fix this by deleting the duplicated "Co-authored-by", "Relates-to", "Signed-off-by" fields
```

#### Git Manual Style

The style from the git book, that directly affects the operation of git:

##### subject-not-separated-from-body

If there is a body, enforce a gap between it and the subject.

###### Default status

On an empty repository

``` shell,script(name="subject-not-separated-from-body-default",expected_exit_code=0)
git mit-config lint status subject-not-separated-from-body
```

``` text,verify(script_name="subject-not-separated-from-body-default",stream=stdout)
+---------------------------------+---------+
| Lint                            | Status  |
+===========================================+
| subject-not-separated-from-body | enabled |
+---------------------------------+---------+
```

###### Valid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that is valid
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
Demonstration Commit Message
This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
Demonstration Commit Message
This is a commit message that is invalid


---

Your commit message is missing a blank line between the subject and the body

Most tools that render and parse commit messages, expect commit messages to be in the form of subject and body. This includes git itself in tools like git-format-patch. If you don't include this you may see strange behaviour from git and any related tools.

To fix this separate subject from body with a blank line
```

##### subject-longer-than-72-characters

After 72 characters, git will truncate commit messages in the history
view, this prevents that

###### Default status

On an empty repository

``` shell,script(name="subject-longer-than-72-characters-default",expected_exit_code=0)
git mit-config lint status subject-longer-than-72-characters
```

``` text,verify(script_name="subject-longer-than-72-characters-default",stream=stdout)
+-----------------------------------+---------+
| Lint                              | Status  |
+=============================================+
| subject-longer-than-72-characters | enabled |
+-----------------------------------+---------+
```

###### Valid

Using message

``` shell,file(path="message")
cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc

Demonstration Commit Message
This is a commit message that is valid
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc

Demonstration Commit Message
This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc

Demonstration Commit Message
This is a commit message that is invalid


---

Your subject is longer than 72 characters

It's important to keep the subject of the commit less than 72 characters because when you look at the git log, that's where it truncates the message. This means that people won't get the entirety of the information in your commit.

Please keep the subject line 72 characters or under
```

##### body-wider-than-72-characters

After 72 characters, git will truncate commit messages in the history
view, this prevents that

###### Default status

On an empty repository

``` shell,script(name="body-wider-than-72-characters-default",expected_exit_code=0)
git mit-config lint status body-wider-than-72-characters
```

``` text,verify(script_name="body-wider-than-72-characters-default",stream=stdout)
+-------------------------------+---------+
| Lint                          | Status  |
+=========================================+
| body-wider-than-72-characters | enabled |
+-------------------------------+---------+
```

###### Valid

Using message

``` shell,file(path="message")
Demonstration Commit Message

cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
This is a commit message that is valid
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
Demonstration Commit Message

ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
Demonstration Commit Message

ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
This is a commit message that is invalid


---

Your commit has a body wider than 72 characters

It's important to keep the body of the commit narrower than 72 characters because when you look at the git log, that's where it truncates the message. This means that people won't get the entirety of the information in your commit.

You can fix this by making the lines in your body no more than 72 characters
```

#### Git Manual Style Extended

The style from the git book, but that doesn't effect using git

##### subject-line-not-capitalized

Detect a subject line that is not capitalised

###### Default status

On an empty repository

``` shell,script(name="subject-line-not-capitalized-default",expected_exit_code=0)
git mit-config lint status subject-line-not-capitalized
```

``` text,verify(script_name="subject-line-not-capitalized-default",stream=stdout)
+------------------------------+----------+
| Lint                         | Status   |
+=========================================+
| subject-line-not-capitalized | disabled |
+------------------------------+----------+
```

###### Enabling

Enable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint enable subject-line-not-capitalized
```

###### Valid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that is valid
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
demonstration Commit Message

This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
demonstration Commit Message

This is a commit message that is invalid


---

Your commit message is missing a capital letter

The subject line is a title, and as such should be capitalised.

You can fix this by capitalising the first character in the subject
```

###### Disabling

Disable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint disable subject-line-not-capitalized
```

##### subject-line-ends-with-period

Detect a subject line that is not capitalised

###### Default status

On an empty repository

``` shell,script(name="subject-line-ends-with-period-default",expected_exit_code=0)
git mit-config lint status subject-line-ends-with-period
```

``` text,verify(script_name="subject-line-ends-with-period-default",stream=stdout)
+-------------------------------+----------+
| Lint                          | Status   |
+==========================================+
| subject-line-ends-with-period | disabled |
+-------------------------------+----------+
```

###### Enabling

Enable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint enable subject-line-ends-with-period
```

###### Valid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that is valid
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
Demonstration Commit Message.

This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
Demonstration Commit Message.

This is a commit message that is invalid


---

Your commit message ends with a period

It's important to keep your commits short, because we only have a limited number of characters to use (72) before the subject line is truncated. Full stops aren't normally in subject lines, and take up an extra character, so we shouldn't use them in commit message subjects.

You can fix this by removing the period
```

###### Disabling

Disable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint disable subject-line-ends-with-period
```

#### Conventional Commits

##### not-conventional-commit

The conventional changelog is a scheme of commit messages used t

###### Default status

On an empty repository

``` shell,script(name="not-conventional-commits-default",expected_exit_code=0)
git mit-config lint status not-conventional-commit
```

``` text,verify(script_name="not-conventional-commits-default",stream=stdout)
+-------------------------+----------+
| Lint                    | Status   |
+====================================+
| not-conventional-commit | disabled |
+-------------------------+----------+
```

###### Enabling

Enable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint enable not-conventional-commit
```

###### Valid

Using message

``` shell,file(path="message")
fix: correct minor typos in code

see the issue for details

on typos fixed.

Reviewed-by: Z
Refs #133
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
Demonstration Commit Message

This is a commit message that is invalid


---

Your commit message isn't in conventional style

It's important to follow the conventional commit style when creating your commit message. By using this style we can automatically calculate the version of software using deployment pipelines, and also generate changelogs and other useful information without human interaction.

You can fix it by following style

<type>[optional scope]: <description>

[optional body]

[optional footer(s)]

You can read more at https://www.conventionalcommits.org/
```

###### Disabling

Disable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint disable not-conventional-commit
```

#### Issue ID Checks

Check for the presence of issue Ids

##### pivotal-tracker-id-missing

Check for the presence of a Pivotal Tracker ID

###### Default status

On an empty repository

``` shell,script(name="pivotal-tracker-id-missing-default",expected_exit_code=0)
git mit-config lint status pivotal-tracker-id-missing
```

``` text,verify(script_name="pivotal-tracker-id-missing-default",stream=stdout)
+----------------------------+----------+
| Lint                       | Status   |
+=======================================+
| pivotal-tracker-id-missing | disabled |
+----------------------------+----------+
```

###### Enabling

Enable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint enable pivotal-tracker-id-missing
```

###### Valid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that is valid

[#12345884]
# some comment
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
demonstration Commit Message

This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
demonstration Commit Message

This is a commit message that is invalid


---

Your commit message is missing a Pivotal Tracker Id

It's important to add the ID because it allows code to be linked back to the stories it was done for, it can provide a chain of custody for code for audit purposes, and it can give future explorers of the codebase insight into the wider organisational need behind the change. We may also use it for automation purposes, like generating changelogs or notification emails.

You can fix this by adding the Id in one of the styles below to the commit message
[Delivers #12345678]
[fixes #12345678]
[finishes #12345678]
[#12345884 #12345678]
[#12345884,#12345678]
[#12345678],[#12345884]
This will address [#12345884]
```

###### Disabling

Disable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint disable pivotal-tracker-id-missing
```

##### jira-issue-key-missing

Check for the presence of a JIRA Issue Key

###### Default status

On an empty repository

``` shell,script(name="jira-issue-key-missing-default",expected_exit_code=0)
git mit-config lint status jira-issue-key-missing
```

``` text,verify(script_name="jira-issue-key-missing-default",stream=stdout)
+------------------------+----------+
| Lint                   | Status   |
+===================================+
| jira-issue-key-missing | disabled |
+------------------------+----------+
```

###### Enabling

Enable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint enable jira-issue-key-missing
```

###### Valid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that is valid

JRA-123
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
demonstration Commit Message

This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
demonstration Commit Message

This is a commit message that is invalid


---

Your commit message is missing a JIRA Issue Key

It's important to add the issue key because it allows us to link code back to the motivations for doing it, and in some cases provide an audit trail for compliance purposes.

You can fix this by adding a key like `JRA-123` to the commit message
```

###### Disabling

Disable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint disable jira-issue-key-missing
```

##### github-id-missing

Check for the presence of a GitHub ID

###### Default status

On an empty repository

``` shell,script(name="github-id-missing-default",expected_exit_code=0)
git mit-config lint status github-id-missing
```

``` text,verify(script_name="github-id-missing-default",stream=stdout)
+-------------------+----------+
| Lint              | Status   |
+==============================+
| github-id-missing | disabled |
+-------------------+----------+
```

###### Enabling

Enable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint enable github-id-missing
```

###### Valid

Using message

``` shell,file(path="message")
Demonstration Commit Message

This is a commit message that is valid

GH-123
```

Committing will succeed

``` shell,script(name="1",expected_exit_code=0)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

###### Invalid

Using message

``` shell,file(path="message")
demonstration Commit Message

This is a commit message that is invalid
```

Committing will fail

``` shell,script(name="1",expected_exit_code=1)
echo $RANDOM > changes
git add changes
git commit --message="$(cat message)"
```

``` text,verify(script_name="1",stream=stderr)
demonstration Commit Message

This is a commit message that is invalid


---

Your commit message is missing a GitHub ID

It's important to add the issue ID because it allows us to link code back to the motivations for doing it, and because we can help people exploring the repository link their issues to specific bits of code.

You can fix this by adding a ID like the following examples:

#642
GH-642
AnUser/git-mit#642
AnOrganisation/git-mit#642
fixes #642

Be careful just putting '#642' on a line by itself, as '#' is the default comment character
```

###### Disabling

Disable it with

``` shell,script(name="1",expected_exit_code=0)
git mit-config lint disable github-id-missing
```
