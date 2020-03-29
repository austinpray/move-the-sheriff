const importJsx = require('import-jsx');
const React = require('react');
const { useEffect, useState } = React;
const { useApp, Box, Color, useInput, render, Text } = require('ink');
const BigText = require('ink-big-text');
const useStdoutDimensions = require('ink-use-stdout-dimensions');

const { Actor, Cow, Grass, Sherriff } = importJsx('./actor')

const useGrid = () => {
  const [ windowX, windowY ] = useStdoutDimensions();
  return useState(
    Array(windowY)
      .fill(windowY)
      .map(() => Array(windowX).fill(() => <Grass />))
  );
}

/**
 * Move the Sheriff
 */
const MoveTheSheriff = () => {
  const { exit } = useApp();
  const [ windowX, windowY ] = useStdoutDimensions();
  const [x, setX] = React.useState(windowX / 2)
  const [y, setY] = React.useState(windowY / 2)
  const [grid, setGrid] = useGrid();

  useEffect(() => {
    /**
     * Need lodash or something :joy:
     *
     * What needs to happen:
     *  - clear the sherriff pos
     *  - insert the sherrif at his new pos
     *
     * Also this was fun but it will never be performant even
     * if it wasn't so shoddily crafted.
     */
    setGrid([
      ...grid.slice(0, y-1) || null,
      [
        ...grid[y].slice(0, x),
        () => <Sherriff />,
        ...grid[y].slice(x+1) || null,
      ],
      ...grid.slice(y+1) || null,
    ])
  }, [ x, y ])

  useInput((input, key) => {
    if (key.escape) {
      exit()
    }

    if (key.upArrow || input == "w") {
      if (y - 1 < 2) {
        setY(windowY - 2)
      } else {
        setY(y - 1)
      }
    }

    if (key.downArrow || input == "s") {
      if (y + 1 > windowY - 2) {
        setY(2)
      } else {
        setY(y + 1)
      }
    }

    if (key.leftArrow || input == "a") {
      if (x - 1 < 2) {
        setX(windowX - 2)
      } else {
        setX(x - 1)
      }
    }

    if (key.rightArrow || input == "d") {
      if (x + 1 > windowX - 2) {
        setX(2)
      } else {
        setX(x + 1)
      }
    }
  })

  return grid.map((row, rowKey) => (
    <Box key={rowKey} width={windowX}>
      {row.map((Cell, cellKey) =>
        <Cell key={cellKey} />
      )}
    </Box>
  ))
}

render(<MoveTheSheriff/>, { experimental: true });
