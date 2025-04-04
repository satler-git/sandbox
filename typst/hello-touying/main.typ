#import "@preview/touying:0.6.1": *
#import themes.simple: *

#set text(lang: "ja")

#show: simple-theme.with(
  aspect-ratio: "16-9",
  config-common(show-notes-on-second-screen: right),
)

// https://touying-typ.github.io/docs/start

= Title

== First Slide

Hello, Touying!

#pause

Hello, Typst!

#meanwhile // pheaseを一個戻す的な。pheaseはstack的でpop

Meanwhile, #pause we can also use `#meanwhile` to #pause display other content synchronously.

#speaker-note[
  + This is a speaker note.
  + You won't see it unless you use `config-common(show-notes-on-second-screen: right)`
]

