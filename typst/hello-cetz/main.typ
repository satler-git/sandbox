#import "@preview/cetz:0.3.4"
// 途中だけど雰囲気は掴めた

#cetz.canvas({
  import cetz.draw: *

  set-style(
    stroke: 0.4pt,

    grid: (
      stroke: gray + 0.2pt,
      step: 0.5,
    ),
  )
  scale(3)

  grid((-1.5, -1.5), (1.5, 1.5))

  line((-1.5, 0), (1.5, 0))
  line((0, -1.5), (0, 1.5))

  circle((0, 0))

  arc(
    (3mm, 0),
    start: 0deg,
    stop: 30deg,
    radius: 3mm,
    mode: "PIE",
    fill: color.mix((red, 20%), white),
  )
})
