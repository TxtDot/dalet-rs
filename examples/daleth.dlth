# multilines
#
# {text} - input is trimmed with indent
#
# {~n text} - n is number of minimum spaces to add after trimming with indent
# for each line
#
# {#text} - input not modified
#
#
# tag syntax
#
# tag: text body
# tag { multiline text body }
# body text always trimmed
#
# tag [ multiple tags body ]
#
# Arguments
# tag argument
#
# Tags without body and argument also supported
#
#
# custom no tag syntax
#
# {-text} - paragraph, text indent is trimmed
# [[tags]] - element tag with body of multiple tags
# text - element tag with text body

meta "title": Daleth syntax concept
meta "description": This document describes Daleth syntax and some tags

h1: TxtDot revolution
p: TxtDot is a cool project

# If no tag is specified, but "- text" present, then the 'el' tag is placed

- This is element
br

# if no tag is specified but a '{- text}' is present, then the 'p' tag is placed
# '\n' is replaced with ' ' in this format.
{-
    Check Dalet too,
    this is one paragraph
}

{-
    This is another paragraph ({- text\})
}

row "center" [
    link "https://github.com/txtdot/txtdot": Homepage
    btn "https://example.com/donate" [
        # tag without body
        img "https://example.com/donate.png"
        - Donate
    ]
]

# [] for multiple tags
row [
    # if no tag is specified but a '[[]]' is present, then the 'el' tag
    # with multiple tags body placed

    [[
        h2: Features

        ul [
            - Server-side page simplification
            - Media proxy
            - Image compression with Sharp
            - Rendering client-side apps `Vanilla, React, Vue, etc` with webder
            - Search with SearXNG
            - Handy API endpoints
            - No client JavaScript
            - Some kind of Material Design 3
            - Customization with plugins, see @txtdot/sdk and @txtdot/plugins
        ]
    ]]

    [[
        h2: Running

        [[
            h3: Dev

            # {} for multiline strings, indent is automatically trimmed
            code "bash" {
                npm install
                npm run dev
            }

            code "markdown" {~4
                this is codeblock
            }

            # {# Text} Text after "{#" not modified
            code "markdown" {#    this is codeblock}
        ]]

        [[
            h3: Production
            code "bash" {
                npm install
                npm run build
                npm run start
            }
        ]]

        [[
            h3: Docker
            code "bash": docker compose up -d
        ]]

    ]]
]

# Table has custom format
# +| cells | - primary column
#  | cells | - secondary column
#  | Element | Description | - converts to
#  trow [
#    Element
#    Description
#  ]
{> table
    [[ Tag      | Description    | id ]]
     [ h        | Heading        | 0  ]
     [ p        | Paragraph      | 1  ]
     [ img      | Image          | 2  ]
     [ link     | Link           | 3  ]
     [ btn      | Button         | 4  ]
     [ ul       | Unordered list | 5  ]
     [ br       | Line break     | 6  ]
    [[ quantity | 7              | 7  ]]
}
