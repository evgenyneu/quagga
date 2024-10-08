# Default template

This is the default template used by `quagga` to create the output prompt.

## General template structure

The template contains the following sections:

* **Prompt** - Contains the template for the output prompt, specified between the `<prompt>`...`</prompt>` tags. The prompt includes the *header*, *footer* and *file* sections.

* **Header/Footer** - The text that will be placed at the top and bottom of the output prompt. It is specified between the `<header>`...`</header>` and `<footer>`...`</footer>` tags.

* **File** - The content of each individual text file, specified between the `<file>`...`</file>` tags.

* **Multi-part** - Used when the output prompt exceeds the `--max-part-size CHARS` limit, in which case the output is divided into parts, with each part having a part header and footer including the part number and total number of parts. The multipart template is defined between the `<part>`...`</part>` tags. These tags are placed outside the `<prompt>` tags, since they are only used when the output prompt is too large to fit in a single part.

Note: the entire template must enclosed in the opening and closing `template` tags so that `quagga` can locate it in this document.

## Tags

### Header/footer tags

These tags display information that will be shown at the start and the end of the output prompt. They are placed between `<header>`...`</header>` and `<footer>`...`</footer>` tags:

* `<all-file-paths>` - Paths to all files that are included in the output prompt.
* `<tree>` - An ASCII tree representation the file paths.
* `<total-file-size>` - Total size of all files in the output prompt.


### File tags

These tags are related to each individual file included in the output prompt and are placed between `<file>`...`</file>` tags:

* `<file-content>` - The content of the text file.
* `<file-path>` - The path to the file.


### Multi-part tags

These tags are used for indicating the start and the end of each individual part in the multi-part prompt. The template is only used when the output prompt size exceeds the `--max-part-size CHARS` limit. These tags are placed between `<part>`...`</part>` tags:

* `<header>`...`</header>` - The text printed out at the start of each part.
* `<footer>`...`</footer>` - The text printed out at the end of each part.
* `<pending>`...`</pending>` - The text that will be shown when there are more parts remaining. The idea is to tell LLM not to respond until all parts are provided.
* `<part-number>` - The number of the current part.
* `<total-parts>` - The total number of parts.
* `<parts-remaining>` - The number of parts remaining.


## Template

```html
<template>
  <prompt>
    <header>The following is my code:</header>

    <file>
      ------ FILE START <file-path> ------

      <file-content>

      ------ FILE END <file-path> ------
    </file>

    <footer>
      All files:
      <tree>
      Total size: <total-file-size>

      Reminding the important rules:
      * Discuss the code changes first, don't suggest any code changes before we agreed on the approach.
      * Think of an alternative/better way to do what I ask, don't simply follow my instructions.
      * One small code change at a time.
      * All code needs to be tested.
      * Write code in such a way that so it can be used as a library, which also means it needs proper comments and documentation.
      * Focus on code clarity and simplicity, even if it means writing more code (i.e. don't try to be smart or elegant D:).
      * Write small functions that do one thing :D It makes the code simpler and easier to test.
      * No need to show existing code, just the changes.
      * In the response text that is not the code, be very concise.

      What do you think? Let's discuss ideas first without code :D
    </footer>
  </prompt>

  <part>
    <header>
      ======== PART <part-number> OF <total-parts>  ========
    </header>

    <footer>
      ======== END OF PART <part-number>  OF <total-parts>  ========
    </footer>

    <pending>This is only a part of the code. Please do not respond until I provide all parts (<parts-remaining> remaining).</pending>
  </part>
</template>
```
