# Default template

This is the default template used by `quagga` to create the output prompt.

## General template structure

The template contains the following sections:

* **Prompt** - Contains the template for the output prompt, specified between the `{{PROMPT}}`...`{{/PROMPT}}` tags. The prompt includes the *header*, *footer* and *file* sections.

* **Header/Footer** - The text that will be placed at the top and bottom of the output prompt. It is specified between the `{{HEADER}}`...`{{/HEADER}}` and `{{FOOTER}}`...`{{/FOOTER}}` tags.

* **File** - The content of each individual text file, specified between the `{{FILE}}`...`{{/FILE}}` tags.

* **Multi-part** - Used when the output prompt exceeds the `--max-part-size BYTES` limit, in which case the output is divided into parts, with each part having a part header and footer including the part number and total number of parts. The multipart template is defined between the `{{MULTI_PART}}`...`{{/MULTI_PART}}` tags. These tags are placed outside the `{{PROMPT}}` tags, since they are only used when the output prompt is too large to fit in a single part.

Note: the entire template is enclosed in the `TEMPLATE`...`/TEMPLATE` tags (with double curly braces around tag names).

## Tags

### Header/footer tags

These tags display information that will be shown at the start and the end of the output prompt. They are placed between `{{HEADER}}`...`{{/HEADER}}` and `{{FOOTER}}`...`{{/FOOTER}}` tags:

* `{{ALL_FILE_PATHS}}` - Paths to all files that are included in the output prompt.
* `{{TREE}}` - An ASCII tree representation the file paths.
* `{{TOTAL_FILE_SIZE}}` - Total size of all files in the output prompt.


### File tags

These tags are related to each individual file included in the output prompt and are placed between `{{FILE}}`...`{{/FILE}}` tags:

* `{{CONTENT}}` - The content of the text file.
* `{{FILE_PATH}}` - The path to the file.


### Multi-part tags

These tags are used for indicating the start and the end of each individual part in the multi-part prompt. These tags are placed between `{{MULTI_PART}}`...`{{/MULTI_PART}}` tags:

* `{{PART_START}}`...`{{/PART_START}` - The text printed out at the start of each part.
* `{{PART_END}}`...`{{/PART_END}}` - The text printed out at the end of each part.
* `{{PART_NUMBER}}` - The number of the current part.
* `{{TOTAL_PARTS}}` - The total number of parts.
* `{{PARTS_PENDING_MSG}}` - The text that will be shown when there are more parts remaining. The idea is to tell LLM not to respond until all parts are provided.
* `{{PARTS_REMAINING}}` - The number of parts remaining.


## Template

```txt
{{TEMPLATE}}
{{PROMPT}}
{{HEADER}}
The following is my code:
{{/HEADER}}

{{FILE}}
------ FILE START {{FILE_PATH}} ------

{{CONTENT}}

------ {{FILE_PATH}} FILE END ------
{{/FILE}}

{{FOOTER}}
All files:
{{TREE}}
Total size: {{TOTAL_FILE_SIZE}}

Reminding the important rules:
* Discuss the code changes first, don't suggest any code changes before we agreed on the approach.
* Think of an alternative/better way to do what I ask, don't simply follow my instructions.
* One small code change at a time.
* All code needs to be tested.
* Write code in such a way that so it can be used as a library, which also means it needs proper comments and documentation.
* Focus on code clarity and simplicity, even if it means writing more code (i.e. don't try to be smart or elegant D:).
* Write small functions that do one thing :D It makes the code simpler and easier to test.
* In the response text that is not the code, be very concise.

What do you think? Let's discuss ideas first without code :D
{{/FOOTER}}
{{/PROMPT}}

{{MULTI_PART}}
{{PART_START}}======== PART {{PART_NUMBER}} OF {{TOTAL_PARTS}} ======== {{/PART_START}}
{{PART_END}}
======== END OF PART {{PART_NUMBER}} OF {{TOTAL_PARTS}} ========
{{PARTS_PENDING_MSG}}This is only a part of the code. Please do not respond until I provide all parts ({{/PARTS_REMAINING}} remaining)).{{/PARTS_PENDING_MSG}}
{{/PART_END}}
{{/MULTI_PART}}
{{/TEMPLATE}}
```
